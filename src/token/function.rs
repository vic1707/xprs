/* Crate imports */
use crate::token::Identifier;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Function<'a> {
    pub(crate) name: &'a str,
    pub(crate) func: fn(&[f64]) -> f64,
    pub(crate) nb_args: Option<u8>,
}

impl<'a> Function<'a> {
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

    pub const fn new_identifier(
        name: &'a str,
        func: fn(&[f64]) -> f64,
        nb_args: Option<u8>,
    ) -> Identifier<'a> {
        Identifier::Function(Self::new(name, func, nb_args))
    }
}

#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! xprs_function {
    // variadics
    ($function:expr) => {
        Function::new(stringify!($function), move |args| $function(args), None)
    };
    ($function:expr, $nb_args:tt) => {
        Function::new(
            stringify!($function),
            $crate::utils::macros::wrap_into_closure!($function, $nb_args),
            Some($nb_args),
        )
    };
}
