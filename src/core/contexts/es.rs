use std::ffi::CString;

use smart_default::SmartDefault;

#[derive(SmartDefault)]
pub struct ESContext {
    pub version_double: (u8, u8),

    #[default(
        _code = "CString::new(\"Open GL ES 0.0\").unwrap()"
    )]
    pub version: CString,

    #[default(
        _code = "CString::new(\"Maldreno (TM) GPU (Couldn't load GL_RENDERER)\").unwrap()"
    )]
    pub renderer: CString,

    #[default(
        _code = "CString::new(\"0.0 Maldreno\").unwrap()"
    )]
    pub shading_language_version: CString,
}

