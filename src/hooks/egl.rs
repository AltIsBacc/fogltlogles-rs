use crate::{bindings::backend::egl, core::context, register_ov};

register_ov!(
    fn eglMakeCurrent(
        dpy: egl::types::EGLDisplay,
        draw: egl::types::EGLSurface,
        read: egl::types::EGLSurface,
        ctxx: egl::types::EGLContext,
    ) -> egl::types::EGLBoolean => |_ctx| {
		// NOTE: ctx here is still unintialized!
		crate::init();

        let ctx = context::get_global_context()
            .expect("Context failed to initialize!");

        unsafe { ctx.egl.MakeCurrent(dpy, draw, read, ctxx) }
    }
);

