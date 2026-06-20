use std::ffi;

use crate::{bindings::{self, frontend::gl}, define_extension};

define_extension! {
    name: "GL_ARB_blend_func_extended",
    id: ext_blend_func_extended,
    requires: ["GL_EXT_blend_func_extended"],
    exports: [
        fn glBindFragDataLocation(
            program: gl::types::GLuint,
            color_number: gl::types::GLuint,
            name: *const ffi::c_char,
        ) {
            native => bindings::gles() => BindFragDataLocationEXT
        },
        fn glBindFragDataLocationIndexed(
            program: gl::types::GLuint,
            color_number: gl::types::GLuint,
            index: gl::types::GLuint,
            name: *const ffi::c_char,
        ) {
            native => bindings::gles() => BindFragDataLocationIndexedEXT
        }
    ]
}

