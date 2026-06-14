
#[macro_export]
macro_rules! make_mutable {
    ($($arg:ident),*) => {
        $(let mut $arg = $arg;)*
    };
}

