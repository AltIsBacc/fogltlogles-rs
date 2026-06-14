use std::ffi::CString;

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
    fn to_cstr(self) -> CString;
    fn to_cstr_or(self, default: CString) -> CString;
}

impl ToCString for String {
    fn to_cstr(self) -> CString {
        CString::new(self).unwrap()
    }

    fn to_cstr_or(self, default: CString) -> CString {
        CString::new(self).unwrap_or(default)
    }
}

