use crate::{bindings::backend::egl, core::context, register_export};

#[macro_export]
macro_rules! register_egl_passthrough {
    // zero-arg variant
    ($func_name:ident () $( -> $ret_type:ty )? , $method:ident) => {
        $crate::register_export!(
            fn $func_name() $( -> $ret_type )? => |_ctx| {
                crate::init();
                let ctx = $crate::core::context::get_global_context()
                    .expect("Context not initialized");
                unsafe { ctx.egl.$method() }
            }
        );
    };
    // one-or-more-arg variant
    ($func_name:ident ( $( $arg_name:ident : $arg_type:ty ),+ $(,)? ) $( -> $ret_type:ty )? , $method:ident) => {
        $crate::register_export!(
            fn $func_name( $( $arg_name : $arg_type ),+ ) $( -> $ret_type )? => |_ctx| {
                crate::init();
                let ctx = $crate::core::context::get_global_context()
                    .expect("Context not initialized");
                unsafe { ctx.egl.$method($( $arg_name ),+) }
            }
        );
    };
}

register_export!(
    fn eglMakeCurrent(
        dpy: egl::types::EGLDisplay,
        draw: egl::types::EGLSurface,
        read: egl::types::EGLSurface,
        ctxx: egl::types::EGLContext,
    ) -> egl::types::EGLBoolean => |_ctx| {
        crate::init();
        let ctx = context::get_global_context()
            .expect("Context not initialized");
        let result = unsafe { ctx.egl.MakeCurrent(dpy, draw, read, ctxx) };

        crate::main();
        result
    }
);

register_egl_passthrough!(eglGetDisplay(display_id: egl::types::EGLNativeDisplayType) -> egl::types::EGLDisplay, GetDisplay);
register_egl_passthrough!(eglBindAPI(api: egl::types::EGLenum) -> egl::types::EGLBoolean, BindAPI);
register_egl_passthrough!(eglChooseConfig(dpy: egl::types::EGLDisplay, attrib_list: *const egl::types::EGLint, configs: *mut egl::types::EGLConfig, config_size: egl::types::EGLint, num_config: *mut egl::types::EGLint) -> egl::types::EGLBoolean, ChooseConfig);
register_egl_passthrough!(eglGetConfigs(dpy: egl::types::EGLDisplay, configs: *mut egl::types::EGLConfig, config_size: egl::types::EGLint, num_config: *mut egl::types::EGLint) -> egl::types::EGLBoolean, GetConfigs);
register_egl_passthrough!(eglGetConfigAttrib(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, attribute: egl::types::EGLint, value: *mut egl::types::EGLint) -> egl::types::EGLBoolean, GetConfigAttrib);
register_egl_passthrough!(eglGetError() -> egl::types::EGLint, GetError);
register_egl_passthrough!(eglInitialize(dpy: egl::types::EGLDisplay, major: *mut egl::types::EGLint, minor: *mut egl::types::EGLint) -> egl::types::EGLBoolean, Initialize);
register_egl_passthrough!(eglTerminate(dpy: egl::types::EGLDisplay) -> egl::types::EGLBoolean, Terminate);
register_egl_passthrough!(eglCreateContext(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, share_context: egl::types::EGLContext, attrib_list: *const egl::types::EGLint) -> egl::types::EGLContext, CreateContext);
register_egl_passthrough!(eglDestroyContext(dpy: egl::types::EGLDisplay, ctx_: egl::types::EGLContext) -> egl::types::EGLBoolean, DestroyContext);
register_egl_passthrough!(eglDestroySurface(dpy: egl::types::EGLDisplay, surface: egl::types::EGLSurface) -> egl::types::EGLBoolean, DestroySurface);
register_egl_passthrough!(eglCreateWindowSurface(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, win: egl::types::EGLNativeWindowType, attrib_list: *const egl::types::EGLint) -> egl::types::EGLSurface, CreateWindowSurface);
register_egl_passthrough!(eglCreatePbufferSurface(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, attrib_list: *const egl::types::EGLint) -> egl::types::EGLSurface, CreatePbufferSurface);
register_egl_passthrough!(eglSwapBuffers(dpy: egl::types::EGLDisplay, surface: egl::types::EGLSurface) -> egl::types::EGLBoolean, SwapBuffers);
register_egl_passthrough!(eglSwapInterval(dpy: egl::types::EGLDisplay, interval: egl::types::EGLint) -> egl::types::EGLBoolean, SwapInterval);
register_egl_passthrough!(eglQueryString(dpy: egl::types::EGLDisplay, name: egl::types::EGLint) -> *const std::ffi::c_char, QueryString);

