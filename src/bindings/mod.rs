use std::{ffi, sync};

use crate::{bindings::backend::{egl, gles2}, utils::sync::UnsafeSendSync};

pub mod types;
pub mod backend;
pub mod frontend;

static GLES: sync::OnceLock<UnsafeSendSync<gles2::Gles2>> = sync::OnceLock::new();
static EGL: sync::OnceLock<UnsafeSendSync<egl::Egl>> = sync::OnceLock::new();

pub fn load_apis<F>(mut loader: F)
where
    F: FnMut(&str) -> *const ffi::c_void,
{
    let gles = gles2::Gles2::load_with(|s| loader(s));
    let egl = egl::Egl::load_with(|s| loader(s));

    GLES.set(UnsafeSendSync(gles)).ok();
    EGL.set(UnsafeSendSync(egl)).ok();
}

pub fn gles() -> &'static UnsafeSendSync<gles2::Gles2>{
    &GLES.get().expect("GLES dispatch not initialized")
}

pub fn egl() -> &'static UnsafeSendSync<egl::Egl> {
    &EGL.get().expect("EGL dispatch not initialized")
}

