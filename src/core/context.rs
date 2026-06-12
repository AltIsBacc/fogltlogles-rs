use std::ffi::CString;

#[derive(Default)]
pub struct FogleContext {
    pub es_version: (u8, u8),

    pub gl_version: CString,
    pub gl_renderer: CString,
}

impl FogleContext {
    pub fn new() -> Self {
        Self::default()
    }
}

pub mod management {
    use std::{
        cell::Cell,
        collections::HashMap,
        sync::LazyLock,
    };

    use parking_lot::Mutex;

    use crate::bindings::backend::egl;

    use super::*;

    type EGLContextHandle = usize;

    struct ContextPtr(*mut FogleContext);
    unsafe impl Send for ContextPtr {}

    static CONTEXT_MAP: LazyLock<Mutex<HashMap<EGLContextHandle, ContextPtr>>> = 
        LazyLock::new(|| Mutex::new(HashMap::new()));

    thread_local! {
        static CURRENT_CTX: Cell<*mut FogleContext> = Cell::new(std::ptr::null_mut());
    }

    pub fn register(handle: egl::types::EGLContext) {
        let ptr = Box::into_raw(Box::new(FogleContext::new()));
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

    /// Returns `Some(&FogleContext)` current on this thread,
    /// or None if no context is bound.
    #[inline(always)]
    #[allow(dead_code)]
    pub fn current() -> Option<&'static mut FogleContext> {
        CURRENT_CTX.with(|p| {
            let raw = p.get();
            if raw.is_null() { None } else {
                // SAFETY:
                // - Box<FogleContext> has a stable address; HashMap resizes never
                //   move existing boxes.
                // - EGL guarantees a context is not destroyed while current, so
                //   the box outlives this reference.
                // - thread_local access means only this thread reads this pointer;
                //   no concurrent &mut aliasing is possible
                Some(unsafe { &mut *raw })
            }
        })
    }

    #[inline]
    #[allow(dead_code)]
    pub fn with_current<'a, F, R>(runnable: F) -> R
    where 
        F: FnOnce(&'a mut FogleContext) -> R
    {
        CURRENT_CTX.with(|p| runnable(unsafe {
            &mut *p.get()
        }))
    }
}

pub mod macros {
    /// Returns a [`&FogleContext`] for the context bound to the current thread.
    ///
    /// # Panics
    /// Panics if no context has been bound on the calling thread via [`management::bind`].
    #[macro_export]
    macro_rules! current_ctx {
        () => {
            $crate::core::context::management::current().
                expect("No context initialized on the current thread!")
        };
    }

    /// Returns [`Some(&FogleContext)`] for the context bound to the
    /// current thread, or None if no context is bound.
    #[macro_export]
    macro_rules! try_current_ctx {
        () => {
            $crate::core::context::management::current()
        };
    }
}

