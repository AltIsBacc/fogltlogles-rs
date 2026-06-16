use std::collections::HashMap;

use anyhow::Result;

use crate::{bindings::frontend::gl, core::shader::transpilation::TranspileContext, traits::gl::{AsStageKey, FromStageKey}};

pub mod transpilation;
pub mod cache;

#[derive(Default)]
pub struct Shader {
    pub id: gl::types::GLuint,
    pub type_: gl::types::GLenum,
    pub source: String,
    pub spirv: Vec<u8>,

    pub compiled: bool,
}

impl Shader {
    pub fn new(id: gl::types::GLuint, type_: gl::types::GLenum) -> Self {
        Self {
            id,
            type_,
            ..Default::default()
        }
    }

    pub fn compile_source(&mut self) {
        match transpilation::glsl_to_spv(&self.source, self.type_) {
            Ok(spv) => {
                self.spirv = spv;
                self.compiled = true;
            }
            Err(e) => {
                log::error!("glsl_to_spv failed: {e}");
            }
        }
    }
}

#[derive(Default)]
pub struct Program {
    pub stages: [ShaderStage; 6],
    pub transpile_context: TranspileContext,

    pub linked: bool,
}

impl Program {
    pub fn attach(&mut self, shader: &Shader) -> Result<()> {
        let stage = self.stages.get_mut(
            shader.type_.as_stage_key()?
        ).ok_or_else(|| anyhow::anyhow!("out of bounds?"))?;
        stage.shaders.push(shader.id);

        Ok(())
    }

    pub fn link_stages(&mut self, shaders: &HashMap<gl::types::GLuint, Shader>) 
        -> Result<Vec<(gl::types::GLenum, String)>> // (stage_type, merged_essl) per active stage
    {
        let mut result = Vec::new();

        for (i, stage) in self.stages.iter().enumerate() {
            if stage.shaders.is_empty() { continue; }

            let mut merged_essl = String::new();
            for shader_id in &stage.shaders {
                let shader = shaders.get(shader_id).unwrap();
                let essl = transpilation::spv_to_essl(
                    &shader.spirv,
                    shader.type_,
                    &mut self.transpile_context,
                )?;
                merged_essl.push_str(&essl);
            }

            result.push((
                i.from_stage_key()
                    .ok_or_else(|| anyhow::anyhow!("invalid index!"))?,
                merged_essl
            ));
        }

        Ok(result)
    }
}

#[derive(Default)]
pub struct ShaderStage {
    pub shaders: Vec<gl::types::GLuint>, // shader id
}

#[derive(Default)]
pub struct PipelineState {
    pub shaders: HashMap<gl::types::GLuint, Shader>,
    pub programs: HashMap<gl::types::GLuint, Program>,
}

impl PipelineState {
    pub fn new_shader(&mut self, shader_id: gl::types::GLuint, type_: gl::types::GLenum) {
        self.shaders.insert(shader_id, Shader::new(shader_id, type_));
    }

    pub fn new_program(&mut self, program_id: gl::types::GLuint) {
        self.programs.insert(program_id, Default::default());
    }
}

