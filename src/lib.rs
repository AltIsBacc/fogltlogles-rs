use std::ffi;

use multi_log::MultiLogger;
use parking_lot::Once;

use crate::{bindings::backend::gles2, traits::ffi::{ToCStr, ToCString}};

pub(crate) mod utils;
pub(crate) mod traits;
pub(crate) mod bindings;

pub(crate) mod core;
pub(crate) mod api;
pub(crate) mod ffpe;
pub(crate) mod hooks;

static INIT_ONCE: Once = Once::new();
static LATE_INIT_ONCE: Once = Once::new();

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
            .for_each(|f| unsafe { (f.init)() });
    });
}

fn ensure_requirements() {
    let version = unsafe {
        bindings::gles().GetString(gles2::VERSION).to_cstr()
    };

    let (major, minor) = version.to_str().unwrap()
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

    let ctx = current_ctx!();
    ctx.es.version_double = (major.try_into().unwrap(), minor.try_into().unwrap());
    ctx.es.version = version.to_cstring();
    ctx.es.renderer = unsafe {
        bindings::gles().GetString(gles2::RENDERER).to_cstring()
    };
    
}

