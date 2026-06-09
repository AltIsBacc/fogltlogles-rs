use std::sync;

use android_logger::Config;

use crate::core::context;

pub(crate) mod bindings;

pub(crate) mod core;
pub(crate) mod api;
pub(crate) mod ffpe;
pub(crate) mod hooks;

static ONCE: sync::Once = sync::Once::new();

pub fn init() {
    android_logger::init_once(
        Config::default()
        .with_tag("fogltlogles-rs")
    );

    ONCE.call_once(|| {
        context::set_global_context(Box::new(
            context::FogleContext::new()
        ));

        log::info!("loaded!");
    });
}

