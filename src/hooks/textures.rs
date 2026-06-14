use crate::{bindings::{self, frontend::gl}, core::formats, make_mutable, register_hook};

register_hook! {
    fn glTexImage2D(
        target: gl::types::GLenum,
        level: gl::types::GLint,
        internalformat: gl::types::GLint,
        width: gl::types::GLsizei,
        height: gl::types::GLsizei,
        border: gl::types::GLint,
        format: gl::types::GLenum,
        type_: gl::types::GLenum,
        pixels: *const gl::types::GLvoid,
    ) => {
        make_mutable!(internalformat, format, type_);

        log::info!(
            "glTexImage2D : internalformat={} border={} format={} type={}",
            internalformat, border, format, type_
        );

        formats::texture::fix_tex_image_2d(
            unsafe {
                &mut *(&mut internalformat as *mut gl::types::GLint as *mut gl::types::GLenum)
            },
            &mut format,
            &mut type_
        );

        bindings::gles().TexImage2D(
            target, level, internalformat,
            width, height,
            border, format, type_,
            pixels
        )
    }
}

