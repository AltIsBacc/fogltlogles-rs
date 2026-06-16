use std::ffi::CString;
use smart_default::SmartDefault;

use crate::{bindings::frontend::gl, core::shader};

#[derive(SmartDefault)]
pub struct FogleContext {
    #[default(
        _code = "CString::new(\"4.5 (on ES 0.0)\").unwrap()"
    )]
    pub version: CString,                                                                     
    
    #[default(
        _code = "CString::new(\"FOGLTLOGLES 0.0 (on Unknown)\").unwrap()"
    )]
    pub renderer: CString,

    pub current_error: gl::types::GLbitfield,

    pub shader_pipeline: shader::PipelineState,
}

impl FogleContext {
   pub fn raise_error(&mut self, error: gl::types::GLenum) {
        if let Some(bit) = Self::error_to_bit_index(error) {
            self.current_error |= 1 << bit;
        }
    }

    pub fn get_error(&mut self) -> gl::types::GLenum {
        if self.current_error == 0 {
            return gl::NO_ERROR; 
        }

        let bit_index = self.current_error.trailing_zeros();
        let detected_error = Self::bit_index_to_error(bit_index);

        self.current_error &= !(1 << bit_index);

        detected_error
    }

    fn error_to_bit_index(error: gl::types::GLenum) -> Option<u32> {
        match error {
            gl::INVALID_ENUM => Some(0),
            gl::INVALID_VALUE => Some(1),
            gl::INVALID_OPERATION => Some(2),
            gl::STACK_OVERFLOW => Some(3),
            gl::STACK_UNDERFLOW => Some(4),
            gl::OUT_OF_MEMORY => Some(5),
            gl::INVALID_FRAMEBUFFER_OPERATION => Some(6),
            _ => None,
        }
    }

    fn bit_index_to_error(index: u32) -> gl::types::GLenum {
        match index {
            0 => gl::INVALID_ENUM,
            1 => gl::INVALID_VALUE,
            2 => gl::INVALID_OPERATION,
            3 => gl::STACK_OVERFLOW,
            4 => gl::STACK_UNDERFLOW,
            5 => gl::OUT_OF_MEMORY,
            6 => gl::INVALID_FRAMEBUFFER_OPERATION,
            _ => unreachable!("Internal error bitfield corruption"),
        }
    } 

}

