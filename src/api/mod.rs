use std::{env, ffi, ptr, sync::OnceLock};

pub mod macros;
pub mod loaders;

use libloading::os::unix::{Library, RTLD_LAZY, RTLD_LOCAL};

pub struct InterceptEntry {
    pub name: &'static str,
    pub ptr: *const ffi::c_void,
}

unsafe impl Sync for InterceptEntry { }

#[linkme::distributed_slice]
pub static INTERCEPT_REGISTRY: [InterceptEntry];

pub struct InterceptInitEntry {
    pub init: fn(), 
}

unsafe impl Sync for InterceptInitEntry { }

#[linkme::distributed_slice]
pub static INTERCEPT_INIT_REGISTRY: [InterceptInitEntry];

static EGL_LIB: OnceLock<Library> = OnceLock::new();
static EGL_GET_PROC: OnceLock<
    unsafe extern "C" fn(*const ffi::c_char) -> *const ffi::c_void
> = OnceLock::new();

pub fn egl_lib() -> &'static Library {
    EGL_LIB.get_or_init(|| {
        let lib_path = &env::var("LIBGL_EGL").unwrap_or("libEGL.so".into());

        // SAFETY: loading libEGL is inherently unsafe (runs its init routines),
        // but we just use it for symbol resolution so it's fine
        let lib = unsafe {
            Library::open(Some(lib_path), RTLD_LAZY | RTLD_LOCAL)
                .unwrap_or_else(|_| panic!("Failed to open {}", lib_path))
        };

        // Load eglGetProcAddress while we have the library handle.
        match unsafe {
            lib.get::<unsafe extern "C" fn(*const ffi::c_char) -> *const ffi::c_void>(
                b"eglGetProcAddress\0",
            )
        } {
            Ok(sym) => {
                let _ = EGL_GET_PROC.set(*sym);
            },
            Err(e) => {
                log::warn!("Failed to load eglGetProcAddress from libEGL.so! You may encounter some issues..");
                log::warn!("{}", e);
            }
        }

        lib
    })
}

pub fn fogle_get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    crate::init(); // this is the main entrypoint of the renderer
    if name.is_null() {
        return ptr::null();
    }

    let c_str = unsafe { ffi::CStr::from_ptr(name) };
    if let Ok(s) = c_str.to_str() {
        for entry in INTERCEPT_REGISTRY.iter() {
            if entry.name == s {
                log::info!("Overriding function : {}", s);
                return entry.ptr;
            }
        }
    }

    egl_get_proc_address(name)
}

pub fn egl_get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() {
        return ptr::null();
    }

    let lib = egl_lib(); // trigger lazy load
    unsafe {
        // Try eglGetProcAddress first.
        if let Some(f) = EGL_GET_PROC.get() {
            let ptr = f(name);
            if !ptr.is_null() {
                return ptr;
            }

            log::trace!(
                "Failed to load function : {} (via: eglGetProcAddress)",
                ffi::CStr::from_ptr(name).to_str().unwrap()
            );
        }

        match lib.get::<unsafe extern "C" fn()>(std::slice::from_raw_parts(
            name as *const u8,
            ffi::CStr::from_ptr(name).to_bytes_with_nul().len(),
        )) {
            Ok(sym) => *sym as *const ffi::c_void,
            Err(_) => {
                log::trace!(
                    "Failed to get function : {} (via: dlsym)",
                    ffi::CStr::from_ptr(name).to_str().unwrap()
                );

                ptr::null()
            },
        }
    }
}

