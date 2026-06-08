#![allow(unsafe_op_in_unsafe_fn)]

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

