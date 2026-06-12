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

