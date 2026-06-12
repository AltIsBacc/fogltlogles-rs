#[macro_export]
macro_rules! register_export {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[unsafe(no_mangle)]
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? $body

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
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr, init: $init_body:expr) => {
        paste::paste! {
            $crate::register_export!(fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? => $body);

            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            unsafe fn [<init_ $func_name>]() $init_body

            #[allow(non_snake_case)]
            mod [<$func_name _init_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_INIT_REGISTRY)]
                static __INIT_ENTRY: $crate::api::InterceptInitEntry = $crate::api::InterceptInitEntry {
                    init: super::[<init_ $func_name>],
                };
            }
        }
    };
}

#[macro_export]
macro_rules! register_fn {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? $body 

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
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr, init: $init_body:expr) => {
        paste::paste! {
            $crate::register_fn!(fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? => $body);

            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            unsafe fn [<init_ $func_name>]() $init_body

            #[allow(non_snake_case)]
            mod [<$func_name _init_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_INIT_REGISTRY)]
                static __INIT_ENTRY: $crate::api::InterceptInitEntry = $crate::api::InterceptInitEntry {
                    init: super::[<init_ $func_name>],
                };
            }
        }
    };
}

#[macro_export]
macro_rules! register_hook {
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr) => {
        paste::paste! {
            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            pub unsafe extern "C" fn [<ov_ $func_name>]( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? $body

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
    (fn $func_name:ident ( $( $arg_name:ident : $arg_type:ty ),* $(,)? ) $( -> $ret_type:ty )? => $body:expr, init: $init_body:expr) => {
        paste::paste! {
            $crate::register_hook!(fn $func_name( $( $arg_name : $arg_type ),* ) $( -> $ret_type )? => $body);

            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            unsafe fn [<init_ $func_name>]() $init_body

            #[allow(non_snake_case)]
            mod [<$func_name _ov_init_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_INIT_REGISTRY)]
                static __INIT_ENTRY: $crate::api::InterceptInitEntry = $crate::api::InterceptInitEntry {
                    init: super::[<init_ $func_name>],
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
    ($name:ident => $target:ident, init: $init_body:expr) => {
        paste::paste! {
            $crate::register_redir!($name => $target);

            #[allow(non_snake_case)]
            #[allow(unsafe_op_in_unsafe_fn)]
            unsafe fn [<init_ $func_name>]() $init_body

            #[allow(non_snake_case)]
            mod [<$name _redir_init_entry>] {
                #[linkme::distributed_slice($crate::api::INTERCEPT_INIT_REGISTRY)]
                static __INIT_ENTRY: $crate::api::InterceptInitEntry = $crate::api::InterceptInitEntry {
                    init: super::[<init_ $func_name>],
                };
            }
        }
    };
}

