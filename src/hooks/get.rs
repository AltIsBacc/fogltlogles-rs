use crate::{bindings, register_ov};

register_ov!(
    fn glGetIntegerv(
        pname: u32,
        data: *mut i32,
    ) =>{
        log::info!("glGetIntegerv : pname={:#X}", pname);

        bindings::gles().GetIntegerv(pname, data);
    }
);

