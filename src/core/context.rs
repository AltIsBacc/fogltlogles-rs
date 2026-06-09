use std::{ffi, sync::atomic::{AtomicPtr, Ordering}};
use crate::{api, bindings::backend::{egl, gles2}, ffpe};

pub struct FogleContext {
    pub gles: gles2::Gles2,
    pub egl: egl::Egl,
    pub ffpe: ffpe::context::FogleFFPEContext,
}

impl FogleContext {
    pub fn new() -> Self {
        Self {
            gles: Self::load_gles(),
            egl: Self::load_egl(),
            ffpe: ffpe::context::FogleFFPEContext::new(),
        }
    }

    fn load_gles() -> gles2::Gles2 {
        let loader = |name| {
            let cname = ffi::CString::new(name).unwrap();
            api::gles_get_proc_address(cname.as_ptr())
        };

        gles2::Gles2::load_with(loader)
    }

    fn load_egl() -> egl::Egl {
        let loader = |name| {
            let cname = ffi::CString::new(name).unwrap();
            api::egl_get_proc_address(cname.as_ptr())
        };

        egl::Egl::load_with(loader)
    }
}

// Global atomic pointer holding the context. 
// Kept internal (no pub) so it cannot be altered from outside.
static GLOBAL_CTX: AtomicPtr<FogleContext> = AtomicPtr::new(std::ptr::null_mut());

pub fn set_global_context(ctx: Box<FogleContext>) {
    let ptr = Box::into_raw(ctx);
    let old_ptr = GLOBAL_CTX.swap(ptr, Ordering::SeqCst);
    if !old_ptr.is_null() {
        unsafe { drop(Box::from_raw(old_ptr)); }
    }
}

#[inline(always)]
pub fn get_global_context_raw() -> *mut FogleContext {
    GLOBAL_CTX.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn get_global_context() -> Option<&'static FogleContext> {
    let ctx_ptr = get_global_context_raw();

    if ctx_ptr.is_null() { return None; }

    let ctx = unsafe {
        &*ctx_ptr
    };

    Some(ctx)
}

