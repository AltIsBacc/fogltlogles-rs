use crate::{bindings::{self, frontend::gl}, register_hook};



register_hook!(
    fn glGetIntegerv(
        pname: u32,
        data: *mut i32,
    ) => {
        log::info!("glGetIntegerv : pname={:#X}", pname);

        match pname {
            gl::MAJOR_VERSION => *data = 4,
            gl::MINOR_VERSION => *data = 5,

            gl::CONTEXT_FLAGS => *data = gl::NONE as i32,
            gl::CONTEXT_PROFILE_MASK => *data = gl::CONTEXT_COMPATIBILITY_PROFILE_BIT as i32,

            _ => bindings::gles().GetIntegerv(pname, data),
        };
    }
);

register_hook!(
    fn glGetString(
        pname: u32,
    ) -> *const u8 => {
        match pname {
            gl::VERSION => c"4.5 (on ES {}.{})".as_ptr(),
            gl::SHADING_LANGUAGE_VERSION => c"4.00 FOGLTLOGLES".as_ptr(),

            _ => bindings::gles().GetString(pname)
        }
    },
    init: {
    }
);

