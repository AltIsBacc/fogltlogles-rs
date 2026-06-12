
#[allow(unsafe_op_in_unsafe_fn)]
pub mod egl {
    pub use crate::bindings::types::egl::*;

    include!(concat!(env!("OUT_DIR"), "/backend_egl.rs"));
}

#[allow(unsafe_op_in_unsafe_fn)]
pub mod gles2 {
    include!(concat!(env!("OUT_DIR"), "/backend_gles2.rs"));
}

