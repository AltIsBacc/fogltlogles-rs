use std::{ffi, ptr};

pub mod loaders;

pub struct InterceptEntry {
    pub name: &'static str,
    pub ptr: *const ffi::c_void,
}

unsafe impl Sync for InterceptEntry {}

#[linkme::distributed_slice]
pub static INTERCEPT_REGISTRY: [InterceptEntry];

#[macro_export]
macro_rules! register_func {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? {
                let closure = $body;
                closure($crate::core::context::get_global_context())
            }

            #[allow(non_snake_case)]
            mod [<$func_name _entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_REGISTRY)]
                static __INTERCEPT_ENTRY: $crate::api::InterceptEntry = $crate::api::InterceptEntry {
                    name: stringify!($func_name),
                    ptr: super::$func_name as *const std::ffi::c_void,
                };
            }
        }
    };
}


#[macro_export]
macro_rules! register_ov {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn [<ov_ $func_name>]( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? {
                let closure = $body;
                closure($crate::core::context::get_global_context())
            }

            #[allow(non_snake_case)]
            mod [<$func_name _ov_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_REGISTRY)]
                static __INTERCEPT_ENTRY: $crate::api::InterceptEntry = $crate::api::InterceptEntry {
                    name: stringify!($func_name),
                    ptr: super::[<ov_ $func_name>] as *const std::ffi::c_void,
                };
            }
        }
    };
}

#[macro_export]
macro_rules! register_redir {
    ($name:ident => $target:ident) => {
        paste::paste! {
            #[allow(non_snake_case)]
            mod [<$name _redir_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_REGISTRY)]
                static __INTERCEPT_ENTRY: $crate::api::InterceptEntry = $crate::api::InterceptEntry {
                    name: stringify!($name),
                    ptr: super::$target as *const std::ffi::c_void,
                };
            }
        }
    };
}

const RTLD_NEXT: *mut ffi::c_void = -1i64 as *mut ffi::c_void;

pub fn get_proc_address(name: *const ffi::c_char) -> *const ffi::c_void {
    if name.is_null() { return ptr::null(); }
    let c_str = unsafe { ffi::CStr::from_ptr(name) };

    if let Ok(str_slice) = c_str.to_str() {
        for entry in INTERCEPT_REGISTRY.iter() {
            if entry.name == str_slice {
                return entry.ptr;
            }
        }
    }

    get_proc_address_passthrough(name)
}

pub fn get_proc_address_passthrough(name: *const ffi::c_char) -> *const ffi::c_void {
    unsafe { libc::dlsym(RTLD_NEXT, name) }
}

