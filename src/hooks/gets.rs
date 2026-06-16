use std::ffi;

use crate::{bindings::{self, frontend::gl}, current_ctx, register_hook, traits::ffi::FromFmtCString};

register_hook! {
    fn glGetError() -> gl::types::GLenum => {
        let ctx = current_ctx!();

        let err = ctx.fogle.get_error();
        if err != gl::NO_ERROR {
            return err;
        }
        
        bindings::gles().GetError()
    }
}

register_hook! {
    fn glGetIntegerv(
        pname: gl::types::GLenum,
        data: *mut gl::types::GLint,
    ) => {
        log::info!("glGetIntegerv : pname={:#X}", pname);

        match pname {
            gl::MAJOR_VERSION => *data = 3,
            gl::MINOR_VERSION => *data = 3,

            gl::CONTEXT_FLAGS => *data = gl::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT as i32,
            gl::CONTEXT_PROFILE_MASK => *data = gl::CONTEXT_CORE_PROFILE_BIT as i32,

            gl::NUM_EXTENSIONS => *data = 0,

            _ => bindings::gles().GetIntegerv(pname, data),
        };
    }
}

register_hook! {
    fn glGetString(
        pname: gl::types::GLenum,
    ) -> *const gl::types::GLubyte => {
        let ctx = current_ctx!();
        match pname {
            gl::VERSION => ctx.fogle.version.as_ptr(),
            gl::RENDERER => ctx.fogle.renderer.as_ptr(),
            gl::VENDOR => c"ThatMG393, AltIsBacc".as_ptr(),

            gl::SHADING_LANGUAGE_VERSION => c"4.00 FOGLTLOGLES".as_ptr(),
            _ => bindings::gles().GetString(pname),
        }
    },
    init: {
        let ctx = current_ctx!();

        ctx.fogle.version = ffi::CString::from_fmt(
            format_args!("4.5 (on ES {}.{})", ctx.es.version_double.0, ctx.es.version_double.1)
        );

        ctx.fogle.renderer = ffi::CString::from_fmt(
            format_args!(
                "FOGLTLOGLES {} (on {})",
                "crate::build_info::get_version()",
                ctx.es.renderer.to_str().unwrap()
            )
        );
    }
}

