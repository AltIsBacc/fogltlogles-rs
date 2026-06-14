use std::{cell::RefCell, collections::HashMap};

use crate::bindings::frontend::gl;

pub mod transpilation;

#[derive(Default)]
pub struct ShaderInfo {
    pub shader_type: gl::types::GLenum,
    pub source: String,
}

#[derive(Default)]
pub struct TranspileState {
    pub bindings: HashMap<String, u32>,
    pub next_binding: u32,
}

#[derive(Default)]
pub struct ProgramInfo {
    pub stages: HashMap<gl::types::GLenum, Vec<String>>,
    pub driver_shaders: HashMap<gl::types::GLenum, gl::types::GLuint>,
    pub attached_shaders: HashMap<gl::types::GLuint, gl::types::GLuint>,
    pub transpile_state: TranspileState,
    pub linked: bool,
}

#[derive(Default)]
pub struct PipelineState {
    pub shaders: HashMap<gl::types::GLuint, ShaderInfo>,
    pub programs: HashMap<gl::types::GLuint, ProgramInfo>,
}

thread_local! {
    pub static PIPELINE_STATE: RefCell<PipelineState> = RefCell::new(Default::default());
}

