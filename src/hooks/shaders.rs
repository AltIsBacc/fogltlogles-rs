use crate::{bindings::{self, frontend::gl}, core::{shader::cache}, current_ctx, register_hook, traits::{ffi::ToCString}, utils};

register_hook! {
    fn glCreateShader(shader_type: gl::types::GLenum) -> gl::types::GLuint => {
        let ctx = current_ctx!();
        let shader_id = bindings::gles().CreateShader(shader_type);

        ctx.fogle.shader_pipeline.new_shader(shader_id, shader_type);
        shader_id
    }
}

register_hook! {
    fn glCreateProgram() -> gl::types::GLuint => {
        let ctx = current_ctx!();
        let program_id = bindings::gles().CreateProgram();

        ctx.fogle.shader_pipeline.new_program(program_id);
        program_id
    }
}

register_hook! {
    fn glShaderSource(
        shader_id: gl::types::GLuint,
        count: gl::types::GLsizei,
        string: *const *const gl::types::GLchar,
        length: *const gl::types::GLint,
    ) => {
        let ctx = current_ctx!();
        let Some(shader) = ctx.fogle.shader_pipeline.shaders.get_mut(&shader_id) else {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        };

        shader.source = utils::ffi::combine_gl_strings(count, string, length);
    }
}

register_hook! {
    fn glCompileShader(
        shader_id: gl::types::GLuint
    ) => {
        let ctx = current_ctx!();
        let Some(shader) = ctx.fogle.shader_pipeline.shaders.get_mut(&shader_id) else {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        };

        let spv_key = cache::keys::spv_key(&shader);

        match cache::get_spv(spv_key) {
            Ok(spv) => {
                shader.spirv = spv;
                shader.compiled = true;
            }
            Err(_) => {
                shader.compile_source();
                if shader.compiled {
                    if let Err(e) = cache::put_spv(spv_key, &shader.spirv) {
                        log::warn!("spv cache write failed: {e}");
                    }
                }
            }
        }
    }
}

register_hook! {
    fn glAttachShader(
        program_id: gl::types::GLuint,
        shader_id: gl::types::GLuint,
    ) => {
        let ctx = current_ctx!();
        let Some(program) = ctx.fogle.shader_pipeline.programs.get_mut(&program_id) else {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        };

        let Some(shader) = ctx.fogle.shader_pipeline.shaders.get_mut(&shader_id) else {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        };

        if let Err(_) = program.attach(&shader) {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        }
    }
}


register_hook! {
    fn glLinkProgram(
        program_id: gl::types::GLuint,
    ) => {
        let ctx = current_ctx!();
        
        let Some(program) = ctx.fogle.shader_pipeline.programs.get_mut(&program_id) else {
            ctx.fogle.raise_error(gl::INVALID_VALUE);
            return;
        };

        let program_hash = cache::keys::program_key(&ctx.es, &program);
        if let Ok(entry) = cache::get_program(program_hash) {
            program.transpile_context = entry.transpile_context;
            bindings::gles().ProgramBinary(
                program_id,
                entry.format,
                entry.binary.as_ptr() as *const _,
                entry.binary.len() as gl::types::GLint,
            );
            program.linked = true;
            return;
        }

        let Ok(linked_stages) = program.link_stages(&ctx.fogle.shader_pipeline.shaders) else {
            ctx.fogle.raise_error(gl::INVALID_OPERATION);
            return;
        };

        for (stage_type, merged_essl) in linked_stages {
            let gl_shader = bindings::gles().CreateShader(stage_type);
            let essl_cstring = merged_essl.to_cstring();
            let ptr = essl_cstring.as_ptr();

            bindings::gles().ShaderSource(gl_shader, 1, &ptr, std::ptr::null());
            bindings::gles().CompileShader(gl_shader);
            bindings::gles().AttachShader(program_id, gl_shader);
        }        

        bindings::gles().LinkProgram(program_id);

        let mut linked: gl::types::GLint = 0;
        bindings::gles().GetProgramiv(program_id, gl::LINK_STATUS, &mut linked);

        program.linked = if linked == gl::TRUE as gl::types::GLint { true } else { false };

        let mut length: gl::types::GLint = 0;
        bindings::gles().GetProgramiv(program_id, gl::PROGRAM_BINARY_LENGTH, &mut length);

        let mut format: gl::types::GLenum = 0;
        let mut binary = vec![0u8; length as usize];
        bindings::gles().GetProgramBinary(
            program_id,
            length,
            std::ptr::null_mut(),
            &mut format,
            binary.as_mut_ptr() as *mut _,
        );

        if let Err(e) = cache::put_program(program_hash, &cache::ProgramCacheEntry {
            format,
            binary,
            transpile_context: program.transpile_context.clone(),
        }) {
            log::warn!("progbin cache write failed: {e}");
        }
    }
}

register_hook! {
    fn glGetShaderiv(
        shader: gl::types::GLuint,
        pname: gl::types::GLenum,
        params: *mut gl::types::GLint,
    ) => {
        match pname {
            gl::COMPILE_STATUS => {
                let ctx = current_ctx!();
                *params = if ctx.fogle.shader_pipeline.shaders
                    .get(&shader)
                    .map_or(false, |s| s.compiled)
                {
                    gl::TRUE as gl::types::GLint
                } else {
                    gl::FALSE as gl::types::GLint
                };
            }
            _ => bindings::gles().GetShaderiv(shader, pname, params)
        }
    }
}

register_hook! {
    fn glGetProgramiv(
        program: gl::types::GLuint,
        pname: gl::types::GLenum,
        params: *mut gl::types::GLint,
    ) => {
        match pname {
            gl::LINK_STATUS => {
                let ctx = current_ctx!();
                *params = if ctx.fogle.shader_pipeline.programs
                    .get(&program)
                    .map_or(false, |p| p.linked)
                {
                    gl::TRUE as gl::types::GLint
                } else {
                    gl::FALSE as gl::types::GLint
                };
            }
            _ => bindings::gles().GetProgramiv(program, pname, params)
        }
    }
}

