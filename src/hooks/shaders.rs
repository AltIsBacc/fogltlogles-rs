use std::hash::Hasher;

use gxhash::GxHasher;

use crate::{bindings::{self, frontend::gl}, core, current_ctx, register_hook, traits::ffi::ToCString, utils};

register_hook! {
    fn glCreateShader(shader_type: gl::types::GLenum) -> gl::types::GLuint => {
        let shader = bindings::gles().CreateShader(shader_type);

        core::shader::PIPELINE_STATE.with_borrow_mut(|ps| {
            ps.shaders.insert(shader, core::shader::ShaderInfo {
                shader_type,
                ..Default::default()
            });
        });

        shader
    }
}

register_hook! {
    fn glShaderSource(
        shader: gl::types::GLuint,
        count: gl::types::GLsizei,
        string: *const *const gl::types::GLchar,
        length: *const gl::types::GLint,
    ) => {
        core::shader::PIPELINE_STATE.with_borrow_mut(|ps| {
            if let Some(info) = ps.shaders.get_mut(&shader) {
                info.source = utils::ffi::combine_gl_strings(count, string, length);
            }
        });
    }
}

register_hook! {
    fn glCompileShader(
        _shader: gl::types::GLuint
    ) => {
        // no-op
    }
}

register_hook! {
    fn glAttachShader(
        program: gl::types::GLuint,
        shader: gl::types::GLuint,
    ) => {
        core::shader::PIPELINE_STATE.with_borrow_mut(|ps| {
            if let Some(info) = ps.shaders.get(&shader) {
                let shader_type = info.shader_type;
                let source = info.source.clone();

                if let Some(program_info) = ps.programs.get_mut(&program) {
                    program_info.stages.entry(shader_type).or_default().push(source);

                    // create driver shader and track app -> driver mapping
                    let driver_shader = *program_info.driver_shaders.entry(shader_type).or_insert_with(|| {
                        let s = bindings::gles().CreateShader(shader_type);
                        bindings::gles().AttachShader(program, s);
                        s
                    });

                    program_info.attached_shaders.insert(shader, driver_shader);
                }
            }
        });
    }
}

register_hook! {
    fn glCreateProgram() -> gl::types::GLuint => {
        let program = bindings::gles().CreateProgram();

        core::shader::PIPELINE_STATE.with_borrow_mut(|ps| {
            ps.programs.insert(program, core::shader::ProgramInfo::default());
        });

        program
    }
}

register_hook! {
    fn glLinkProgram(
        program: gl::types::GLuint,
    ) => {
        let ctx = current_ctx!();
        core::shader::PIPELINE_STATE.with_borrow_mut(|ps| {
            let Some(program_info) = ps.programs.get_mut(&program) else {
                ctx.fogle.raise_error(gl::INVALID_VALUE);
                return;
            };

            let mut hasher = GxHasher::default();

            let mut active_types = [0u32; 6];
            let mut count = 0;
            for &shader_type in program_info.stages.keys() {
                if count < active_types.len() {
                    active_types[count] = shader_type;
                    count += 1;
                }
            }

            active_types[..count].sort_unstable();

            for &shader_type in &active_types[..count] {
                let sources = &program_info.stages[&shader_type];
                let full_stage_source = sources.join("\n");

                hasher.write(full_stage_source.as_bytes());

                let driver_shader = program_info.driver_shaders[&shader_type];

                let converted = core::shader::transpilation::transpile(
                    &full_stage_source, shader_type, &mut program_info.transpile_state
                ).expect("Shader transpilation failed");

                let c_str = converted.to_cstr();
                bindings::gles().ShaderSource(driver_shader, 1, &c_str.as_ptr(), std::ptr::null());
                bindings::gles().CompileShader(driver_shader);

                let mut status = 0i32;
                bindings::gles().GetShaderiv(driver_shader, gl::COMPILE_STATUS, &mut status);
                if status != gl::TRUE as i32 {
                    log::error!("Shader compilation failed for type {}", shader_type);
                    ctx.fogle.raise_error(gl::INVALID_OPERATION);
                    return;
                }
            }

            let _cache_key = hasher.finish();

            bindings::gles().LinkProgram(program);

            let mut status = 0i32;
            bindings::gles().GetProgramiv(program, gl::LINK_STATUS, &mut status);
            if status != gl::TRUE as i32 {
                log::error!("Program linking failed for program {}", program);
                ctx.fogle.raise_error(gl::INVALID_OPERATION);
                return;
            }

            program_info.linked = true;
        });
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
                core::shader::PIPELINE_STATE.with_borrow(|ps| {
                    unsafe {
                        *params = if ps.shaders.get(&shader).map_or(false, |s| !s.source.is_empty()) {
                            gl::TRUE as gl::types::GLint
                        } else {
                            gl::FALSE as gl::types::GLint
                        };
                    }
                });
            }
            _ => unsafe { bindings::gles().GetShaderiv(shader, pname, params) }
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
                core::shader::PIPELINE_STATE.with_borrow(|ps| {
                    unsafe {
                        *params = if ps.programs.get(&program).map_or(false, |p| p.linked) {
                            gl::TRUE as gl::types::GLint
                        } else {
                            gl::FALSE as gl::types::GLint
                        };
                    }
                });
            }
            _ => unsafe { bindings::gles().GetProgramiv(program, pname, params) }
        }
    }
}

