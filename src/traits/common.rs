use std::ffi::CStr;

use crate::bindings::frontend::gl;

pub trait ToStr<'a> {
    fn to_str(self) -> &'a str;
    fn to_str_or(self, default: &'a str) -> &'a str;
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

    fn to_str_or(self, default: &'a str) -> &'a str {
        if self.is_null() {
            panic!("Pointer points to nothing!");
        }

        unsafe {
            CStr::from_ptr(self).to_str().unwrap_or(default)
        }
    }
}

