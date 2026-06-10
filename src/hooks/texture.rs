use std::ffi;

use crate::{bindings, core::context::FogleContext, register_ov};

register_ov! {
    fn glTexImage2D(
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: *const ffi::c_void,
    ) => |ctx: Option<&'_ mut FogleContext>| {
        bindings::gles().TexImage2D(
            target, level, internalformat,
            width, height,
            border, format, type_,
            pixels
        )
    }
}

