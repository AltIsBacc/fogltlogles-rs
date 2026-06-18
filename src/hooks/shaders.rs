use crate::{bindings::{self, frontend::gl}, core::shader::cache, current_ctx, register_hook, traits::gl::AsGLBool, utils};

register_hook! {
    fn glCreateShader(shader_type: gl::types::GLenum) -> gl::types::GLuint => {
        let ctx = current_ctx!();
        ctx.fogle.shader_pipeline.new_shader(shader_type)
    }
}

register_hook! {
    fn glCreateProgram() -> gl::types::GLuint => {
        let ctx = current_ctx!();
        ctx.fogle.shader_pipeline.new_program()
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

        shader.set_source(utils::ffi::combine_gl_strings(count, string, length));
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
        let Ok(spv) = cache::get_spv(spv_key) else {
            if !shader.compile_source() {
                return;
            };

            if let Err(e) = cache::put_spv(spv_key, &shader.spirv()) {
                log::warn!("spv cache write failed: {e}");
            }

            return;
        };
        
        shader.load_from_cache(&spv);
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
            program.restore_and_link_from_entry(&entry);
            return;
        }

        let Ok(()) = program.build_stages(&ctx.fogle.shader_pipeline.shaders) else {
            return;
        };

        program.link();

        let Some((format, binary)) = program.get_program_binary() else {
            return;
        };

        if let Err(e) = cache::put_program(program_hash, &cache::ProgramCacheEntry {
            format,
            binary,
            transpile_context: program.take_transpile_context(),
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
                *params = ctx.fogle.shader_pipeline.shaders
                    .get(&shader)
                    .map_or(false, |s| s.compiled())
                    .as_gl_bool() as gl::types::GLint;
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
                *params = ctx.fogle.shader_pipeline.programs
                    .get(&program)
                    .map_or(false, |p| p.linked())
                    .as_gl_bool() as gl::types::GLint;
            }
            _ => bindings::gles().GetProgramiv(program, pname, params)
        }
    }
}

