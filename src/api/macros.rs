
#[macro_export]
macro_rules! register_export {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[unsafe(no_mangle)]
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? {
                let closure = $body;
                closure($crate::core::context::current_context())
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
macro_rules! register_func {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? {
                let closure = $body;
                closure($crate::core::context::current_context())
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
                closure($crate::core::context::current_context())
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

