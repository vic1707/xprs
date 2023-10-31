#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function<'a> {
    pub(crate) name: &'a str,
    pub(crate) func: fn(&[f64]) -> f64,
    pub(crate) nb_args: Option<u8>,
}

impl<'a> Function<'a> {
    #[inline]
    #[must_use]
    pub const fn new(
        name: &'a str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Self {
        Self {
            name,
            func,
            nb_args,
        }
    }
}

#[macro_export]
macro_rules! xprs_fn {
    // variadics
    ($name:literal, $function:expr) => {
        $crate::token::Function::new($name, $function, None)
    };
    ($function:expr) => {
        $crate::token::Function::new(stringify!($function), $function, None)
    };
    // fixed args
    ($name:literal, $function:expr, $nb_args:tt) => {
        $crate::token::Function::new(
            $name,
            $crate::utils::macros::wrap_into_closure!($function, $nb_args),
            Some($nb_args),
        )
    };
    ($function:expr, $nb_args:tt) => {
        $crate::token::Function::new(
            stringify!($function),
            $crate::utils::macros::wrap_into_closure!($function, $nb_args),
            Some($nb_args),
        )
    };
}
