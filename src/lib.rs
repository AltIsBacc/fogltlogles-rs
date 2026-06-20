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
pub(crate) mod extensions;

static INIT_ONCE: Once = Once::new();
static LATE_INIT_EARLY_ONCE: Once = Once::new();
static LATE_INIT_LATE_ONCE: Once = Once::new();

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
    let mut version_info = (Default::default(), (42, 42));
    LATE_INIT_EARLY_ONCE.call_once(|| {
        version_info = ensure_requirements();
    });

    let ctx = current_ctx!();
    ctx.es.version_double = version_info.1;
    ctx.es.version = version_info.0;
    ctx.es.renderer = unsafe {
        bindings::gles().GetString(gles2::RENDERER).to_cstring()
    };
    ctx.es.shading_language_version = unsafe {
        bindings::gles().GetString(gles2::SHADING_LANGUAGE_VERSION).to_cstring()
    };

    let mut ext_count: gles2::types::GLint = 0;
    unsafe { bindings::gles().GetIntegerv(gles2::NUM_EXTENSIONS, &mut ext_count) };

    for i in 0..ext_count as gles2::types::GLuint {
        let ext_ptr = unsafe { bindings::gles().GetStringi(gles2::EXTENSIONS, i) };
        if !ext_ptr.is_null() {
            if let Ok(s) = unsafe { ffi::CStr::from_ptr(ext_ptr) }.to_str() {
                ctx.es.real_extensions.insert(s.to_owned());
            }
        }
    }

    log::info!("FOGLTLOGLES: found {} real extensions", ctx.es.real_extensions.len());

    ctx.fogle.fake_extensions = ctx.es.real_extensions.clone();

    LATE_INIT_LATE_ONCE.call_once(|| {
        api::INTERCEPT_INIT_REGISTRY.iter()
            .for_each(|f| unsafe { (f.init)() });

        extensions::EXTENSION_REGISTRY.iter()
            .for_each(|e| (e.register)());
    });
}

fn ensure_requirements() -> (ffi::CString, (u8, u8)) {
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

    (version.to_cstring(), (major.try_into().unwrap(), minor.try_into().unwrap()))
}

