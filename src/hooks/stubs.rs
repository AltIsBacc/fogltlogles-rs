use crate::{bindings::frontend::gl, register_stub};

register_stub!(glPolygonMode(_face: gl::types::GLenum, _mode: gl::types::GLenum));

