use std::{ffi, sync};

use multi_log::MultiLogger;

use crate::bindings::backend::gles2;

pub(crate) mod bindings;
pub(crate) mod errors;

pub(crate) mod core;
pub(crate) mod api;
pub(crate) mod ffpe;
pub(crate) mod hooks;

static INIT_ONCE: sync::Once = sync::Once::new();
static LATE_INIT_ONCE: sync::Once = sync::Once::new();

pub fn init() {
    INIT_ONCE.call_once(|| {
        MultiLogger::init(
            vec![Box::new(
                env_logger::Logger::from_default_env()
            ), Box::new(
                android_logger::AndroidLogger::new(
                    android_logger::Config::default()
                )
            )],
            log::Level::Info
        ).unwrap();

        bindings::load_apis(|name| {
            let cname = ffi::CString::new(name).unwrap();
            let ptr = api::egl_get_proc_address(cname.as_ptr());
            if ptr.is_null() {
                log::error!("Failed to load GLES function named : {}", name);
            }

            ptr
        });
    });
}

pub fn late_init() {
    LATE_INIT_ONCE.call_once(|| {
        ensure_requirements();

        api::INTERCEPT_INIT_REGISTRY.iter()
            .for_each(|f| (f.init)());
    });
}

fn ensure_requirements() {
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

    current_ctx!().version = (major as u8, minor as u8);
}

