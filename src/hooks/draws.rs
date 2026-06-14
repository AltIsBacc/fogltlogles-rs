use crate::{bindings::{self, frontend::gl}, register_fn};

register_fn! {
    fn glDrawBuffer(buf: gl::types::GLenum) => {
        bindings::gles().DrawBuffers(1, &buf);
    }
}

