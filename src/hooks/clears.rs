use crate::{bindings::frontend::gl, register_passthrough};

register_passthrough!(glClearDepth(depth: gl::types::GLdouble as gl::types::GLfloat), gles::ClearDepthf);

