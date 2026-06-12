use std::ffi;

use crate::{bindings::{self, backend::gles2, frontend::gl}, current_ctx, register_hook, traits::{common::ToStr, ffi::FromFmtCString}};

register_hook!(
    fn glGetIntegerv(
        pname: u32,
        data: *mut i32,
    ) => {
        log::info!("glGetIntegerv : pname={:#X}", pname);

        match pname {
            gl::MAJOR_VERSION => *data = 4,
            gl::MINOR_VERSION => *data = 5,

            gl::CONTEXT_FLAGS => *data = gl::NONE as i32,
            gl::CONTEXT_PROFILE_MASK => *data = gl::CONTEXT_COMPATIBILITY_PROFILE_BIT as i32,

            gl::NUM_EXTENSIONS => *data = 0,

            _ => bindings::gles().GetIntegerv(pname, data),
        };
    }
);

register_hook!(
    fn glGetString(
        pname: u32,
    ) -> *const gl::types::GLubyte => {
        let ctx = current_ctx!();
        match pname {
            gl::VERSION => ctx.gl_version.as_ptr(),
            gl::RENDERER => ctx.gl_renderer.as_ptr(),

            gl::SHADING_LANGUAGE_VERSION => c"4.00 FOGLTLOGLES".as_ptr(),
            _ => bindings::gles().GetString(pname),
        }
    },
    init: {
        let ctx = current_ctx!();

        ctx.gl_version = ffi::CString::from_fmt(
            format_args!("4.5 (on ES {}.{})", ctx.es_version.0, ctx.es_version.1)
        );

        ctx.gl_renderer = ffi::CString::from_fmt(
            format_args!(
                "FOGLTLOGLES {} (on {})",
                "future crate::build_info::get_version() coming soon near you!!11!!",
                bindings::gles().GetString(gles2::VERSION).to_str()
            )
        );
    }
);

