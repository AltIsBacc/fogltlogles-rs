use crate::bindings::frontend::gl;


pub fn fix_tex_image_2d(
    internalformat: &mut gl::types::GLenum,
    format: &mut gl::types::GLenum,
    type_: &mut gl::types::GLenum,
) {
    match *internalformat {
        gl::DEPTH_COMPONENT32 => {
            *internalformat = gl::DEPTH_COMPONENT32F;
        }

        _ => { }
    };
}

