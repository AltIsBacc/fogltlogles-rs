use std::{ffi, ptr, sync::atomic};

pub mod macros;
pub mod loaders;

pub struct InterceptEntry {
    pub name: &'static str,
    pub ptr: *const ffi::c_void,
}

unsafe impl Sync for InterceptEntry {}

#[linkme::distributed_slice]
pub static INTERCEPT_REGISTRY: [InterceptEntry];

static EGL_HANDLE: atomic::AtomicPtr<ffi::c_void> = atomic::AtomicPtr::new(ptr::null_mut());
static GLES_HANDLE: atomic::AtomicPtr<ffi::c_void> = atomic::AtomicPtr::new(ptr::null_mut());
static EGL_GET_PROC: atomic::AtomicPtr<ffi::c_void> = atomic::AtomicPtr::new(ptr::null_mut());

fn open_lib(cell: &atomic::AtomicPtr<ffi::c_void>, name: &ffi::CStr) -> *mut ffi::c_void {
    let mut handle = cell.load(atomic::Ordering::Relaxed);
    if handle.is_null() {
        handle = unsafe { libc::dlopen(name.as_ptr(), libc::RTLD_NOW | libc::RTLD_GLOBAL) };
        cell.store(handle, atomic::Ordering::Relaxed);
    }
    handle
}

pub fn egl_handle() -> *mut ffi::c_void {
    let mut handle = EGL_HANDLE.load(atomic::Ordering::Relaxed);
    if handle.is_null() {
        handle = open_lib(&EGL_HANDLE, c"libEGL.so");
        EGL_HANDLE.store(handle, atomic::Ordering::Relaxed);

        // load eglGetProcAddress once while we have the handle
        let get_proc = unsafe { libc::dlsym(handle, c"eglGetProcAddress".as_ptr()) };
        EGL_GET_PROC.store(get_proc, atomic::Ordering::Relaxed);
    }
    handle
}

pub fn gles_handle() -> *mut ffi::c_void { open_lib(&GLES_HANDLE, c"libGLESv2.so") }

fn get_egl_get_proc() -> Option<unsafe extern "C" fn(*const ffi::c_char) -> *const ffi::c_void> {
    let ptr = EGL_GET_PROC.load(atomic::Ordering::Relaxed);
    if ptr.is_null() { return None; }
    Some(unsafe { std::mem::transmute(ptr) })
}

fn intercept_lookup(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() { return ptr::null(); }

    let c_str = unsafe { ffi::CStr::from_ptr(name) };

    if let Ok(s) = c_str.to_str() {
        for entry in INTERCEPT_REGISTRY.iter() {
            if entry.name == s {
                return entry.ptr;
            }
        }
    }

    ptr::null()
}

pub fn fogle_get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() { return ptr::null(); }

    let ptr = intercept_lookup(name);
    if !ptr.is_null() { return ptr; }

    let ptr = gles_get_proc_address(name);
    if !ptr.is_null() { return ptr; }

    egl_get_proc_address(name)
}

pub fn gles_get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() { return ptr::null(); }
    unsafe { libc::dlsym(gles_handle(), name) }
}

pub fn egl_get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() { return ptr::null(); }
    unsafe {
        if let Some(f) = get_egl_get_proc() {
            let ptr = f(name);
            if !ptr.is_null() { return ptr; }
        }

        libc::dlsym(egl_handle(), name)
    }
}

