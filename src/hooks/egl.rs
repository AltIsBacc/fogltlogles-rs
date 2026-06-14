use crate::{bindings::{self, backend::egl}, core::contexts, register_fn, register_passthrough};

register_fn! {
    fn eglCreateContext(
        dpy: egl::types::EGLDisplay,
        config: egl::types::EGLConfig,
        share_context: egl::types::EGLContext,
        attrib_list: *const egl::types::EGLint,
    ) -> egl::types::EGLContext => {
        let result = unsafe {
           bindings::egl().CreateContext(dpy, config, share_context, attrib_list)
        };

        if !result.is_null() {
            contexts::management::register(result);
        }

        result
    }
}

register_fn! {
    fn eglDestroyContext(
        dpy: egl::types::EGLDisplay,
        ctx: egl::types::EGLContext,
    ) -> egl::types::EGLBoolean => {
        let result = unsafe {
            bindings::egl().DestroyContext(dpy, ctx)
        };

        if result != 0 {
            contexts::management::unregister(ctx);
        }

        result
    }
}

register_fn! {
    fn eglMakeCurrent(
        dpy: egl::types::EGLDisplay,
        draw: egl::types::EGLSurface,
        read: egl::types::EGLSurface,
        ctx: egl::types::EGLContext,
    ) -> egl::types::EGLBoolean => {
        let result = unsafe {
            bindings::egl().MakeCurrent(dpy, draw, read, ctx)
        };

        if result != 0 {
            contexts::management::bind(ctx);
        }

        result
    }
}

register_passthrough!(eglGetDisplay(display_id: egl::types::EGLNativeDisplayType) -> egl::types::EGLDisplay, egl::GetDisplay);
register_passthrough!(eglBindAPI(api: egl::types::EGLenum) -> egl::types::EGLBoolean, egl::BindAPI);
register_passthrough!(eglChooseConfig(dpy: egl::types::EGLDisplay, attrib_list: *const egl::types::EGLint, configs: *mut egl::types::EGLConfig, config_size: egl::types::EGLint, num_config: *mut egl::types::EGLint) -> egl::types::EGLBoolean, egl::ChooseConfig);
register_passthrough!(eglGetConfigs(dpy: egl::types::EGLDisplay, configs: *mut egl::types::EGLConfig, config_size: egl::types::EGLint, num_config: *mut egl::types::EGLint) -> egl::types::EGLBoolean, egl::GetConfigs);
register_passthrough!(eglGetConfigAttrib(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, attribute: egl::types::EGLint, value: *mut egl::types::EGLint) -> egl::types::EGLBoolean, egl::GetConfigAttrib);
register_passthrough!(eglGetError() -> egl::types::EGLint, egl::GetError);
register_passthrough!(eglInitialize(dpy: egl::types::EGLDisplay, major: *mut egl::types::EGLint, minor: *mut egl::types::EGLint) -> egl::types::EGLBoolean, egl::Initialize);
register_passthrough!(eglTerminate(dpy: egl::types::EGLDisplay) -> egl::types::EGLBoolean, egl::Terminate);
register_passthrough!(eglDestroySurface(dpy: egl::types::EGLDisplay, surface: egl::types::EGLSurface) -> egl::types::EGLBoolean, egl::DestroySurface);
register_passthrough!(eglCreateWindowSurface(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, win: egl::types::EGLNativeWindowType, attrib_list: *const egl::types::EGLint) -> egl::types::EGLSurface, egl::CreateWindowSurface);
register_passthrough!(eglCreatePbufferSurface(dpy: egl::types::EGLDisplay, config: egl::types::EGLConfig, attrib_list: *const egl::types::EGLint) -> egl::types::EGLSurface, egl::CreatePbufferSurface);
register_passthrough!(eglSwapBuffers(dpy: egl::types::EGLDisplay, surface: egl::types::EGLSurface) -> egl::types::EGLBoolean, egl::SwapBuffers);
register_passthrough!(eglSwapInterval(dpy: egl::types::EGLDisplay, interval: egl::types::EGLint) -> egl::types::EGLBoolean, egl::SwapInterval);
register_passthrough!(eglQueryString(dpy: egl::types::EGLDisplay, name: egl::types::EGLint) -> *const std::ffi::c_char, egl::QueryString);

