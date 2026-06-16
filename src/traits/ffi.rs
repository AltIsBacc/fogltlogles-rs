use std::ffi::{CStr, CString};

use crate::bindings::frontend::gl;

pub trait FromFmtCString {
    fn from_fmt(args: std::fmt::Arguments) -> Self;
}

impl FromFmtCString for CString {
    #[inline]
    fn from_fmt(args: std::fmt::Arguments) -> Self {
        let string = std::fmt::format(args);
        CString::new(string).expect("CString format injection encountered an internal null byte")
    }
}

pub trait ToCString {
    fn to_cstring(self) -> CString;
    fn to_cstring_or(self, default: CString) -> CString;
}

impl ToCString for String {
    #[inline]
    fn to_cstring(self) -> CString {
        CString::new(self).unwrap()
    }

    #[inline]
    fn to_cstring_or(self, default: CString) -> CString {
        CString::new(self).unwrap_or(default)
    }
}

impl<'a> ToCString for &'a CStr {
    #[inline]
    fn to_cstring(self) -> CString {
        CString::from(self)
    }

    #[inline]
    fn to_cstring_or(self, _default: CString) -> CString {
        CString::from(self)
    }
}

impl ToCString for *const gl::types::GLubyte {
    #[inline]
    fn to_cstring(self) -> CString {
        if self.is_null() {
            panic!("Pointer points to nothing!");
        }

        CString::from(self.to_cstr())
    }

    #[inline]
    fn to_cstring_or(self, default: CString) -> CString {
        if self.is_null() {
            default
        } else {
            CString::from(self.to_cstr())
        }
    }
}

pub trait ToCStr<'a> {
    fn to_cstr(self) -> &'a CStr;
}

impl<'a> ToCStr<'a> for *const gl::types::GLubyte {
    #[inline]
    fn to_cstr(self) -> &'a CStr {
        if self.is_null() {
            panic!("Pointer points to nothing!");
        }

        unsafe { CStr::from_ptr(self) }
    }
}

