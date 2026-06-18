use std::collections::HashMap;

use anyhow::Result;
use getset::{CopyGetters, Getters};

use crate::{bindings::{self, backend::gles2, frontend::gl}, core::shader::{cache::ProgramCacheEntry, transpilation::TranspileContext}, traits::{ffi::ToCString, gl::{AsStageKey, FromGLBool, FromStageKey}}};

pub mod transpilation;
pub mod cache;

#[derive(Default, Getters, CopyGetters)]
pub struct Shader {
    #[getset(get_copy = "pub")]
    id: gl::types::GLuint,

    #[getset(get_copy = "pub")]
    type_: gl::types::GLenum,

    #[getset(get_copy = "pub")]
    compiled: bool,

    #[getset(get = "pub")]
    source: String,

    #[getset(get = "pub")]
    spirv: Vec<u8>,
}

impl Shader {
    pub fn new(type_: gl::types::GLenum) -> Self {
        Self {
            id: unsafe {
                bindings::gles().CreateShader(type_)
            },
            type_,
            ..Default::default()
        }
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    pub fn compile_source(&mut self) -> bool {
        match transpilation::glsl_to_spv(&self.source, self.type_) {
            Ok(spv) => {
                self.spirv = spv;
                self.compiled = true;

                true
            },
            Err(e) => {
                log::error!("failed to convert shader to spv! {e}");
                self.compiled = false;

                false
            }
        }
    }

    pub fn load_from_cache(&mut self, spv: &Vec<u8>) {
        self.spirv = spv.to_owned();
        self.compiled = true;
    }
}

#[derive(Default)]
pub struct ShaderStage {
    pub shaders: Vec<gl::types::GLuint>, // shader id
}

#[derive(Default, Getters, CopyGetters)]
pub struct Program {
    #[getset(get_copy = "pub")]
    id: gl::types::GLuint,

    #[getset(get_copy = "pub")]
    linked: bool,

    transpile_context: TranspileContext,

    stages: [ShaderStage; 6], // 6 is max shader type
}

impl Program {
    pub fn new() -> Self {
        Self {
            id: unsafe {
                bindings::gles().CreateProgram()
            },
            ..Default::default()
        }
    }

    pub fn attach(&mut self, shader: &Shader) -> Result<()> {
        let stage = self.stages.get_mut(
            shader.type_.as_stage_key()?
        ).ok_or_else(|| anyhow::anyhow!("out of bounds?"))?;
        stage.shaders.push(shader.id);

        Ok(())
    }

    pub fn link(&mut self) {
        unsafe {
            bindings::gles().LinkProgram(self.id);

            let mut linked: gl::types::GLint = 0;
            bindings::gles().GetProgramiv(self.id, gl::LINK_STATUS, &mut linked);

            self.linked = linked.from_gl_bool().unwrap();
        };
    }

    pub fn restore_and_link_from_entry(&mut self, entry: &ProgramCacheEntry) {
        unsafe {
            self.transpile_context = entry.transpile_context.clone();
            bindings::gles().ProgramBinary(
                self.id,
                entry.format,
                entry.binary.as_ptr() as *const _,
                entry.binary.len() as gl::types::GLint,
            );

            let mut linked: gl::types::GLint = 0;
            bindings::gles().GetProgramiv(self.id, gl::LINK_STATUS, &mut linked);

            self.linked = linked.from_gl_bool().unwrap();
        };
    }

    pub fn build_stages(&mut self, shaders: &HashMap<gl::types::GLuint, Shader>) -> Result<()> {
        for (i, stage) in self.stages.iter().enumerate() {
            if stage.shaders.is_empty() { continue; }

            let mut merged_essl = String::new();
            for shader_id in &stage.shaders {
                let shader = shaders.get(shader_id)
                    .ok_or_else(|| anyhow::anyhow!("shader with ID {} not found", shader_id))?;

                let essl = transpilation::spv_to_essl(
                    &shader.spirv,
                    shader.type_,
                    &mut self.transpile_context,
                )?;
                merged_essl.push_str(&essl);
            }

            let gl_shader = unsafe {
                bindings::gles().CreateShader(
                    i.from_stage_key()
                    .ok_or_else(|| anyhow::anyhow!("out of bounds"))?
                )
            };
            
            let essl_cstring = merged_essl.to_cstring();
            unsafe {
                let ptr = essl_cstring.as_ptr();

                bindings::gles().ShaderSource(gl_shader, 1, &ptr, std::ptr::null());
                bindings::gles().CompileShader(gl_shader);

                let mut compiled: gl::types::GLint = 0;
                bindings::gles().GetShaderiv(gl_shader, gles2::COMPILE_STATUS, &mut compiled);

                if !compiled.from_gl_bool().unwrap() {
                    anyhow::bail!("build stages failed!");
                }
                
                bindings::gles().AttachShader(self.id, gl_shader);
                bindings::gles().DeleteShader(gl_shader);
            };
        }

        Ok(())
    }

    pub fn get_program_binary(&mut self) -> Option<(gl::types::GLenum, Vec<u8>)> {
        if !self.linked { return None; }
        unsafe {
            let mut length: gl::types::GLint = 0;
            bindings::gles().GetProgramiv(self.id, gl::PROGRAM_BINARY_LENGTH, &mut length);

            let mut format: gl::types::GLenum = 0;
            let mut binary = vec![0u8; length as usize];
            
            bindings::gles().GetProgramBinary(
                self.id,
                length,
                std::ptr::null_mut(),
                &mut format,
                binary.as_mut_ptr() as *mut _,
            );

            Some((format, binary))
        }
    }

    pub fn take_transpile_context(&mut self) -> TranspileContext {
        std::mem::take(&mut self.transpile_context)
    }
}

#[derive(Default)]
pub struct PipelineState {
    pub shaders: HashMap<gl::types::GLuint, Shader>,
    pub programs: HashMap<gl::types::GLuint, Program>,
}

impl PipelineState {
    pub fn new_shader(&mut self, type_: gl::types::GLenum) -> gl::types::GLuint {
        let shader = Shader::new(type_);
        let shader_id = shader.id;

        self.shaders.insert(shader_id, shader);

        shader_id
    }

    pub fn new_program(&mut self) -> gl::types::GLuint {
        let program = Program::new();
        let program_id = program.id;

        self.programs.insert(program_id, program);

        program_id
    }
}

