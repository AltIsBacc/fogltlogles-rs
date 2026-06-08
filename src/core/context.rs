use std::{ffi, sync::atomic::{AtomicPtr, Ordering}};
use crate::bindings::backend::gles2;

pub struct FogleContext {
    pub api: gles2::Gles2,
}

impl FogleContext {
    pub fn new<F>(load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const ffi::c_void,
    {
        Self {
            api: gles2::Gles2::load_with(load_fn),
        }
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

