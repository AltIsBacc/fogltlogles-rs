use std::{cell::Cell, collections::HashMap, ptr, sync::{LazyLock, Mutex}};
use crate::{bindings::{self, backend::{egl, gles2}}, ffpe};

pub struct FogleContext {
    pub gles: &'static bindings::apis::GlobalDispatch<gles2::Gles2>,
    pub egl:  &'static bindings::apis::GlobalDispatch<egl::Egl>,
    pub ffpe: ffpe::context::FogleFFPEContext,

    ready: bool,  // once flag for ensure_requirements or some other init things
}

impl Default for FogleContext {
    fn default() -> Self { Self::new() }
}

impl FogleContext {
    pub fn new() -> Self {
        Self {
            gles: bindings::apis::gles(),
            egl: bindings::apis::egl(),
            ffpe: ffpe::context::FogleFFPEContext::new(),
            ready: false,
        }
    }

    pub fn init(&mut self) {
        self.ensure_requirements();

        self.ready = true;
    }

    fn ensure_requirements(&self) {
        let version = unsafe {
            let ptr = self.gles.GetString(gles2::VERSION);
            if ptr.is_null() {
                panic!("FOGLTLOGLES: glGetString(GL_VERSION) returned null — no current context?");
            }
            std::ffi::CStr::from_ptr(ptr as *const std::ffi::c_char)
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

type EGLContextHandle = usize;

static CONTEXT_MAP: LazyLock<Mutex<HashMap<EGLContextHandle, Box<FogleContext>>>> = LazyLock::new(||  Mutex::new(HashMap::new()));

thread_local! {
    static CURRENT_CTX: Cell<*mut FogleContext> = Cell::new(ptr::null_mut());
}

pub fn register_context(handle: egl::types::EGLContext) {
    CONTEXT_MAP
        .lock()
        .unwrap()
        .insert(handle as EGLContextHandle, Box::new(FogleContext::new()));
}

pub fn unregister_context(handle: egl::types::EGLContext) {
    CONTEXT_MAP
        .lock()
        .unwrap()
        .remove(&(handle as EGLContextHandle));
}

pub fn bind_context(handle: egl::types::EGLContext) -> bool {
    if handle.is_null() {
        // EGL_NO_CONTEXT
        CURRENT_CTX.with(|p| p.set(ptr::null_mut()));
        return true;
    }

    let mut map = CONTEXT_MAP.lock().unwrap();
    let ctx = match map.get_mut(&(handle as EGLContextHandle)) {
        Some(c) => c,
        None => {
            log::error!("bind_context: untracked EGLContext {:p}", handle);
            return false;
        }
    };

    if !ctx.ready {
        ctx.init();
    }

    CURRENT_CTX.with(|p| p.set(ctx.as_mut() as *mut _));
    true
}

#[inline(always)]
pub fn current_context() -> Option<&'static mut FogleContext> {
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

