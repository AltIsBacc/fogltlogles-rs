
#[macro_export]
macro_rules! define_extension {
    (
        name: $name:literal,
        id: $id:ident,
        $(requires: [ $($req:literal),+ $(,)? ],)?
        exports: [
            $(
                fn $fn_name:ident( $($arg_name:ident : $arg_type:ty),* $(,)? ) $(-> $ret_type:ty)?
                {
                    native => $native_module:expr => $native_name:ident
                    $(, fallback => $fallback:block)?
                }
            ),+ $(,)?
        ]
    ) => {
        paste::paste! {
            pub fn register() {
                let ctx = $crate::current_ctx!();
                const HAS_ANY_FALLBACK: bool = false $(|| { let _ = stringify!($($fallback)?); true })+;
                $($(
                    if !ctx.es.real_extensions.contains($req) {
                        if HAS_ANY_FALLBACK {
                            log::info!(concat!($req, " not present, skipping ", $name, " (fallback available)"));
                        } else {
                            log::info!(concat!($req, " not present, skipping ", $name, " (no fallback)"));
                        }
                        return;
                    }
                )+)?
                ctx.fogle.fake_extensions.insert($name.to_owned());
                log::info!(concat!("Registered extension: ", $name));
            }
            mod [< __ext_registry_ $id >] {
                #[linkme::distributed_slice($crate::extensions::EXTENSION_REGISTRY)]
                static __ENTRY: $crate::extensions::ExtensionEntry = $crate::extensions::ExtensionEntry {
                    register: super::register,
                };
            }
        }
        $(
            $crate::register_hook! {
                fn $fn_name( $($arg_name: $arg_type),* ) $(-> $ret_type)? => {
                    if $native_module.$native_name.is_loaded() {
                        $native_module.$native_name($($arg_name),*)
                    } $(else {
                        $fallback
                    })?
                }
            }
        )+
    };
}

