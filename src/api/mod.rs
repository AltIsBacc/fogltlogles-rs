use std::{ffi, ptr};

pub struct InterceptEntry {
    pub name: &'static str,
    pub ptr: *const ffi::c_void,
}

unsafe impl Sync for InterceptEntry {}

#[linkme::distributed_slice]
pub static INTERCEPT_REGISTRY: [InterceptEntry];

#[macro_export]
macro_rules! register_intercept {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        use $crate::core::context::FogleContext;

        #[allow(non_snake_case)]
        #[allow(unsafe_op_in_unsafe_fn)]
        pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? {
            let ctx_ptr = $crate::core::context::get_global_context_raw();
            
            if ctx_ptr.is_null() {
                #[cfg(debug_assertions)]
                eprintln!(concat!("FOGLTLOGLES: Context uninitialized when calling ", stringify!($func_name)));

                #[cfg(not(debug_assertions))]
                panic!(concat!("FOGLTLOGLES: Context uninitialized when calling ", stringify!($func_name)));
            } else {
                // Safety: Context pointer is checked for null and safely managed globally.
                let ctx = unsafe { &*ctx_ptr };

                let closure = $body;
                closure(ctx)
            }
        }

        paste::paste! {
            #[allow(non_snake_case)]
            mod [<$func_name _intercept>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_REGISTRY)]
                static __INTERCEPT_ENTRY: $crate::api::InterceptEntry = $crate::api::InterceptEntry {
                    name: stringify!($func_name),
                    ptr: super::$func_name as *const std::ffi::c_void,
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

    unsafe { libc::dlsym(RTLD_NEXT, name) }
}

