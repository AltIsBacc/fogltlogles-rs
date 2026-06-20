use anyhow::Result;

use crate::bindings::frontend::gl;

pub trait AsStageKey {
    fn as_stage_key(&self) ->Result<usize>;
}

pub trait FromStageKey  {
    fn from_stage_key(&self) -> Option<gl::types::GLenum>;
}

pub trait AsGLBool {
    fn as_gl_bool(&self) -> gl::types::GLboolean;
}

pub trait FromGLBool {
    fn from_gl_bool(&self) -> Option<bool>;
}

impl AsStageKey for gl::types::GLenum {
    #[inline]
    fn as_stage_key(&self) -> Result<usize> {
        match *self {
            gl::VERTEX_SHADER => Ok(0),
            gl::FRAGMENT_SHADER => Ok(1),
            gl::GEOMETRY_SHADER => Ok(2),
            gl::TESS_CONTROL_SHADER => Ok(3),
            gl::TESS_EVALUATION_SHADER => Ok(4),
            gl::COMPUTE_SHADER => Ok(5),
            other => anyhow::bail!("Unsupported GLenum for shader stage: 0x{other:X}"),
        }
    }
}

impl FromStageKey for usize {
    #[inline]
    fn from_stage_key(&self) -> Option<gl::types::GLenum> {
        match *self {
            0 => Some(gl::VERTEX_SHADER),
            1 => Some(gl::FRAGMENT_SHADER),
            2 => Some(gl::GEOMETRY_SHADER),
            3 => Some(gl::TESS_CONTROL_SHADER),
            4 => Some(gl::TESS_EVALUATION_SHADER),
            5 => Some(gl::COMPUTE_SHADER),
            _ => None,
        }
    }
}

impl AsGLBool for bool {
    #[inline]
    fn as_gl_bool(&self) -> gl::types::GLboolean {
        match *self {
            true => gl::TRUE,
            false => gl::FALSE,
        }
    }
}

impl FromGLBool for gl::types::GLboolean {
    #[inline]
    fn from_gl_bool(&self) -> Option<bool> {
        match *self {
            gl::TRUE => Some(true),
            gl::FALSE => Some(false),
            _ => None
        }
    }
}

impl FromGLBool for gl::types::GLint {
    #[inline]
    fn from_gl_bool(&self) -> Option<bool> {
        match *self as gl::types::GLboolean {
            gl::TRUE => Some(true),
            gl::FALSE => Some(false),
            _ => None
        }
    }
}

