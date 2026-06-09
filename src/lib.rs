use std::sync;

use multi_log::MultiLogger;

use crate::{bindings::backend, core::context};

pub(crate) mod bindings;

pub(crate) mod core;
pub(crate) mod api;
pub(crate) mod ffpe;
pub(crate) mod hooks;

static ONCE: sync::Once = sync::Once::new();

pub fn init() {
    ONCE.call_once(|| {
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

        context::set_global_context(Box::new(
            context::FogleContext::new()
        ));
    });
}

pub fn main() {
    let ctx = context::get_global_context().unwrap();

    let version_str = unsafe {
        let ptr = ctx.gles.GetString(backend::gles2::VERSION);
        if ptr.is_null() {
            panic!("FOGLTLOGLES: Failed to get OpenGL ES version string!");
        }
        std::ffi::CStr::from_ptr(ptr as *const std::ffi::c_char)
            .to_str()
            .expect("Invalid version string!")
    };

    log::info!("GL_VERSION: {}", version_str);

    let mut major = 0i32;
    let mut minor = 0i32;
    unsafe {
        ctx.gles.GetIntegerv(backend::gles2::MAJOR_VERSION, &mut major);
        ctx.gles.GetIntegerv(backend::gles2::MINOR_VERSION, &mut minor);
    }

    if major < 3 || (major == 3 && minor < 2) {
        panic!(
            "FOGLTLOGLES: OpenGL ES 3.2 is required, got {}.{}",
            major, minor
        );
    }

    log::info!("FOGLTLOGLES loaded on ES {}.{}", major, minor);
}

