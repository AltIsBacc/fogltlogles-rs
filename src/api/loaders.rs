
type GenericFuncPtr = unsafe extern "C" fn();

#[cfg(target_os = "windows")]
type GenericWinFuncPtr = unsafe extern "system" fn();

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "android"
))]
pub mod glx_interface {
    use super::GenericFuncPtr;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn glXGetProcAddress(name: *const u8) -> Option<GenericFuncPtr> {
        let ptr = crate::api::fogle_get_proc_address(name as *const std::ffi::c_char);
        unsafe {
            std::mem::transmute::<*const std::ffi::c_void, Option<GenericFuncPtr>>(ptr)
        }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn glXGetProcAddressARB(name: *const u8) -> Option<GenericFuncPtr> {
        unsafe {
            glXGetProcAddress(name)
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "windows")))]
pub mod egl_interface {
    use super::GenericFuncPtr;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn eglGetProcAddress(name: *const std::ffi::c_char) -> Option<GenericFuncPtr> {
        let ptr = crate::api::fogle_get_proc_address(name);
        unsafe {
            std::mem::transmute::<*const std::ffi::c_void, Option<GenericFuncPtr>>(ptr)
        }
    }
}

#[cfg(target_os = "windows")]
pub mod wgl_interface {
    use super::GenericWinFuncPtr;

    #[unsafe(no_mangle)]
    pub unsafe extern "system" fn wglGetProcAddress(name: *const std::ffi::c_char) -> Option<GenericWinFuncPtr> {
        let ptr = crate::api::fogle_get_proc_address(name);
        unsafe {
            std::mem::transmute::<*const std::ffi::c_void, Option<GenericWinFuncPtr>>(ptr)
        }
    }
}

