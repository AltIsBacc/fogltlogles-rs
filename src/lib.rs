use std::{ffi, sync};

use multi_log::MultiLogger;

pub(crate) mod bindings;
pub(crate) mod errors;

pub(crate) mod core;
pub(crate) mod api;
pub(crate) mod ffpe;
pub(crate) mod hooks;

static INIT_ONCE: sync::Once = sync::Once::new();

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

