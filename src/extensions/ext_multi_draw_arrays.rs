use crate::{bindings::{self, frontend::gl}, define_extension};

define_extension! {
    name: "GL_EXT_multi_draw_arrays",
    id: ext_multi_draw_arrays,
    requires: ["GL_EXT_multi_draw_arrays"],
    exports: [
        fn glMultiDrawElementsBaseVertex(
            mode: gl::types::GLenum,
            count: *const gl::types::GLsizei,
            type_: gl::types::GLenum,
            indices: *const *const gl::types::GLvoid,
            drawcount: gl::types::GLsizei,
            basevertex: *const gl::types::GLint,
        ) {
            native => bindings::gles() => MultiDrawElementsBaseVertexEXT,
            fallback => {
                if drawcount <= 0 { return; }

                let counts = unsafe { std::slice::from_raw_parts(count, drawcount as usize) };
                let indices = unsafe { std::slice::from_raw_parts(indices, drawcount as usize) };
                let basevertex = unsafe { std::slice::from_raw_parts(basevertex, drawcount as usize) };

                for i in 0..drawcount as usize {
                    if counts[i] <= 0 { continue; }

                    bindings::gles().DrawElementsBaseVertex(
                        mode,
                        counts[i],
                        type_,
                        indices[i],
                        basevertex[i],
                    );
                }
            }
        }
    ]
}

