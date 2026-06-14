use std::{cell::Cell, collections::HashMap, sync::LazyLock};
use parking_lot::Mutex;

use crate::bindings::backend::egl;

use super::*;

type EGLContextHandle = usize;

struct ContextPtr(*mut TranslationContext);
unsafe impl Send for ContextPtr {}

static CONTEXT_MAP: LazyLock<Mutex<HashMap<EGLContextHandle, ContextPtr>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

thread_local! {
    static CURRENT_CTX: Cell<*mut TranslationContext> = Cell::new(std::ptr::null_mut());
}

pub fn register(handle: egl::types::EGLContext) {
    let ptr = Box::into_raw(Box::new(TranslationContext::new()));
    CONTEXT_MAP
        .lock()
        .insert(handle as EGLContextHandle, ContextPtr(ptr));
}

pub fn unregister(handle: egl::types::EGLContext) {
    if let Some(ContextPtr(ptr)) = CONTEXT_MAP
        .lock()
        .remove(&(handle as EGLContextHandle))
    {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}

pub fn bind(handle: egl::types::EGLContext) -> bool {
    if handle.is_null() {
        CURRENT_CTX.with(|p| p.set(std::ptr::null_mut()));
        return true;
    }

    let ctx = {
        let map = CONTEXT_MAP.lock();
        match map.get(&(handle as EGLContextHandle)) {
            Some(ContextPtr(ptr)) => *ptr,
            None => {
                log::error!("bind: untracked EGLContext {:p}", handle);
                return false;
            }
        }
    };

    CURRENT_CTX.with(|p| p.set(ctx));
    crate::late_init();
    true
}

/// Returns `Some(&TranslationContext)` current on this thread,
/// or None if no context is bound.
#[inline(always)]
#[allow(dead_code)]
pub fn current() -> Option<&'static mut TranslationContext> {
    CURRENT_CTX.with(|p| {
        let raw = p.get();
        if raw.is_null() { None } else {
            Some(unsafe { &mut *raw })
        }
    })
}

#[inline]
#[allow(dead_code)]
pub fn with_current<'a, F, R>(runnable: F) -> R
where 
    F: FnOnce(&'a mut TranslationContext) -> R
{
    CURRENT_CTX.with(|p| runnable(unsafe {
        &mut *p.get()
    }))
}

pub mod macros {
    /// Returns a [`&TranslationContext`] for the context bound to the current thread.
    ///
    /// # Panics
    /// Panics if no context has been bound on the calling thread via [`management::bind`].
    #[macro_export]
    macro_rules! current_ctx {
        () => {
            $crate::core::contexts::management::current().
                expect("No context initialized on the current thread!")
        };
    }

    /// Returns [`Some(&TranslationContext)`] for the context bound to the
    /// current thread, or None if no context is bound.
    #[macro_export]
    macro_rules! try_current_ctx {
        () => {
            $crate::core::contexts::management::current()
        };
    }
}

