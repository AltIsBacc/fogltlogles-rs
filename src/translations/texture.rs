use std::ffi;

use crate::register_intercept;

register_intercept! {
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
    ) => |ctx: &FogleContext| {
        ctx.api.TexImage2D(
            target, level, internalformat,
            width, height,
            border, format, type_,
            pixels
        )
    }
}

