use std::{ffi, ops::Deref, sync};

use crate::bindings::backend::{egl, gles2};

pub mod backend {
    #[allow(unsafe_op_in_unsafe_fn)]
    #[allow(non_camel_case_types)]
    pub mod egl {
        use std::ffi;

        // Types must be at THIS level, direct parent of include!
        pub type khronos_utime_nanoseconds_t = u64;
        pub type khronos_uint64_t = u64;
        pub type khronos_ssize_t = isize;
        pub type EGLNativeDisplayType = *mut ffi::c_void;
        pub type EGLNativePixmapType = *mut ffi::c_void;
        pub type EGLNativeWindowType = *mut ffi::c_void;
        pub type EGLint = i32;
        pub type NativeDisplayType = EGLNativeDisplayType;
        pub type NativePixmapType = EGLNativePixmapType;
        pub type NativeWindowType = EGLNativeWindowType;

        include!(concat!(env!("OUT_DIR"), "/backend_egl.rs"));
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    pub mod gles2 {
        include!(concat!(env!("OUT_DIR"), "/backend_gles2.rs"));
    }
}

/// Wraps a raw-pointer dispatch table for use as a global.
///
/// Safety invariant: populated exactly once at library init (via `OnceLock`),
/// never mutated afterward. The underlying fn pointers are valid to call from
/// any thread that holds a current EGL context.
pub struct GlobalDispatch<T>(T);

unsafe impl<T> Sync for GlobalDispatch<T> {}
unsafe impl<T> Send for GlobalDispatch<T> {}
impl<T> Deref for GlobalDispatch<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

static GLES: sync::OnceLock<GlobalDispatch<gles2::Gles2>> = sync::OnceLock::new();
static EGL: sync::OnceLock<GlobalDispatch<egl::Egl>> = sync::OnceLock::new();

pub fn load_apis<F>(mut loader: F)
where
    F: FnMut(&str) -> *const ffi::c_void,
{
    let gles = gles2::Gles2::load_with(|s| loader(s));
    let egl = egl::Egl::load_with(|s| loader(s));

    GLES.set(GlobalDispatch(gles)).ok();
    EGL.set(GlobalDispatch(egl)).ok();
}

pub fn gles() -> &'static GlobalDispatch<gles2::Gles2>{
    &GLES.get().expect("GLES dispatch not initialized")
}

pub fn egl() -> &'static GlobalDispatch<egl::Egl> {
    &EGL.get().expect("EGL dispatch not initialized")
}

