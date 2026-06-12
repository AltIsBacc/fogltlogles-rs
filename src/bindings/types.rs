use std::ffi::{c_char, c_void};

#[allow(dead_code)]
pub mod gl {
    use super::*;

    pub type GLenum = u32;
    pub type GLboolean = u8;
    pub type GLbitfield = u32;

    pub type GLbyte = i8;
    pub type GLubyte = u8;
    pub type GLshort = i16;
    pub type GLushort = u16;
    pub type GLint = i32;
    pub type GLuint = u32;

    pub type GLsizei = i32;
    pub type GLfloat = f32;
    pub type GLclampf = f32;

    pub type GLint64 = i64;
    pub type GLuint64 = u64;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;

    pub type GLchar = c_char;

    pub type GLsync = *mut c_void;
    pub type GLeglImageOES = *mut c_void;
}

#[allow(non_camel_case_types)]
pub mod egl {
    use super::*;

    pub type khronos_utime_nanoseconds_t = u64;
    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = isize;

    pub type EGLNativeDisplayType = *mut c_void;
    pub type EGLNativePixmapType = *mut c_void;
    pub type EGLNativeWindowType = *mut c_void;
    pub type EGLint = i32;
    pub type NativeDisplayType = EGLNativeDisplayType;
    pub type NativePixmapType = EGLNativePixmapType;
    pub type NativeWindowType = EGLNativeWindowType;
}

