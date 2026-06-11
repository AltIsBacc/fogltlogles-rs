use std::sync::OnceLock;

pub struct FogleContext {
    pub version: (u8, u8),
}

impl Default for FogleContext {
    fn default() -> Self { Self::new() }
}

impl FogleContext {
    pub fn new() -> Self {
        Self {
            version: (0, 0),
        }
    }
}

pub mod management {
    use std::{cell::RefCell, collections::HashMap, sync::{Arc, LazyLock, Mutex}};

    use crate::bindings::backend::egl;

    use super::*;

    type EGLContextHandle = usize;

    static CONTEXT_MAP: LazyLock<Mutex<HashMap<EGLContextHandle, Arc<FogleContext>>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    thread_local! {
        static CURRENT_CTX: RefCell<Option<Arc<FogleContext>>> = const { RefCell::new(None) };
    }

    pub fn register(handle: egl::types::EGLContext) {
        CONTEXT_MAP
            .lock()
            .unwrap()
            .insert(handle as EGLContextHandle, Arc::new(FogleContext::new()));
    }

    pub fn unregister(handle: egl::types::EGLContext) {
        CONTEXT_MAP
            .lock()
            .unwrap()
            .remove(&(handle as EGLContextHandle));
    }

    pub fn bind(handle: egl::types::EGLContext) -> bool {
        if handle.is_null() {
            CURRENT_CTX.with_borrow_mut(|p| *p = None);
            return true;
        }

        let ctx = {
            let map = CONTEXT_MAP.lock().unwrap();
            match map.get(&(handle as EGLContextHandle)) {
                Some(c) => c.clone(),
                None => {
                    log::error!("bind_context: untracked EGLContext {:p}", handle);
                    return false;
                }
            }
        };

        CURRENT_CTX.with_borrow_mut(|p| *p = Some(ctx));

        crate::late_init();

        true
    }

    #[inline(always)]
    pub fn current() -> Option<Arc<FogleContext>> {
        CURRENT_CTX.with_borrow(|p| p.clone())
    }
}

pub mod macros {
    /// Returns the `FogleContext` bound to the current thread.
    ///
    /// # Panics
    /// Panics if no context has been bound on the calling thread via [`core::context::management::bind`].
    #[macro_export]
    macro_rules! current_ctx {
        () => {
            $crate::core::context::management::current()
                .expect("No context initialized on the current thread!")
        };
    }
}

