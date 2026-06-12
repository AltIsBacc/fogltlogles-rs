use std::ffi::CStr;

use crate::bindings::frontend::gl;

pub trait ToStr<'a> {
    fn to_str(self) -> &'a str;
}

impl<'a> ToStr<'a> for *const gl::types::GLubyte {
    #[inline]
    fn to_str(self) -> &'a str {
        if self.is_null() {
            panic!("Pointer points to nothing!");
        }

        unsafe {
            CStr::from_ptr(self).to_str().expect("C String is not a UTF-8")
        }
    }
}

