use std::ffi;
use std::sync::OnceLock;

use crate::bindings::{self, backend::gles2};

pub struct FogleContext {
    initialized: OnceLock<()>,
}

impl Default for FogleContext {
    fn default() -> Self { Self::new() }
}

impl FogleContext {
    pub fn new() -> Self {
        Self {
            initialized: OnceLock::new(),
        }
    }

    fn ensure_init(&self) {
        self.initialized.get_or_init(|| {
            self.ensure_requirements();
        });
    }

    fn ensure_requirements(&self) {
        let version = unsafe {
            let ptr = bindings::gles().GetString(gles2::VERSION);
            if ptr.is_null() {
                panic!("FOGLTLOGLES: glGetString(GL_VERSION) returned null — no current context?");
            }
            ffi::CStr::from_ptr(ptr as *const ffi::c_char)
                .to_str()
                .expect("GL_VERSION is not valid UTF-8")
        };

        let (major, minor) = version
            .strip_prefix("OpenGL ES ")
            .and_then(|s| s.split_whitespace().next())
            .and_then(|v| {
                let mut it = v.splitn(2, '.');
                Some((
                    it.next()?.parse::<u32>().ok()?,
                    it.next()?.parse::<u32>().ok()?,
                ))
            })
            .unwrap_or_else(|| panic!("FOGLTLOGLES: cannot parse GL_VERSION: {:?}", version));

        if major < 3 || (major == 3 && minor < 2) {
            panic!("FOGLTLOGLES: OpenGL ES 3.2 required, got {}.{}", major, minor);
        }

        log::info!("FOGLTLOGLES: context ready on ES {}.{}", major, minor);
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

        ctx.ensure_init();

        CURRENT_CTX.with_borrow_mut(|p| *p = Some(ctx));

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

