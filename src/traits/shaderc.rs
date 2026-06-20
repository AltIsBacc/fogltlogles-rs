use anyhow::{anyhow, Result};

use crate::bindings::frontend::gl;

pub trait FromShaderType: Sized {
    fn from_shader_type(shader_type: gl::types::GLenum) -> Result<Self>;
}

pub trait ToShaderType {
    fn to_shader_type(&self) -> Result<shaderc::ShaderKind>;
}

impl FromShaderType for shaderc::ShaderKind {
    #[inline]
    fn from_shader_type(shader_type: gl::types::GLenum) -> Result<Self> {
        match shader_type {
            gl::VERTEX_SHADER => Ok(shaderc::ShaderKind::Vertex),
            gl::FRAGMENT_SHADER => Ok(shaderc::ShaderKind::Fragment),
            gl::COMPUTE_SHADER => Ok(shaderc::ShaderKind::Compute),
            gl::GEOMETRY_SHADER => Ok(shaderc::ShaderKind::Geometry),
            gl::TESS_CONTROL_SHADER => Ok(shaderc::ShaderKind::TessControl),
            gl::TESS_EVALUATION_SHADER => Ok(shaderc::ShaderKind::TessEvaluation),
            _ => Err(anyhow!("Unsupported shader type: {}", shader_type)),
        }
    }
}

impl ToShaderType for gl::types::GLenum {
    #[inline]
    fn to_shader_type(&self) -> Result<shaderc::ShaderKind> {
        shaderc::ShaderKind::from_shader_type(*self)
    }
}

