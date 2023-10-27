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

pub mod built_in_functions {
    /* Crate imports */
    use super::Function;

    // sin
    pub const SIN: Function = xprs_function!("sin", f64::sin, 1);
    pub const SINH: Function = xprs_function!("sinh", f64::sinh, 1);
    pub const ASIN: Function = xprs_function!("asin", f64::asin, 1);
    pub const ASINH: Function = xprs_function!("asinh", f64::asinh, 1);
    // cos
    pub const COS: Function = xprs_function!("cos", f64::cos, 1);
    pub const COSH: Function = xprs_function!("cosh", f64::cosh, 1);
    pub const ACOS: Function = xprs_function!("acos", f64::acos, 1);
    pub const ACOSH: Function = xprs_function!("acosh", f64::acosh, 1);
    // tan
    pub const TAN: Function = xprs_function!("tan", f64::tan, 1);
    pub const TANH: Function = xprs_function!("tanh", f64::tanh, 1);
    pub const ATAN: Function = xprs_function!("atan", f64::atan, 1);
    pub const ATANH: Function = xprs_function!("atanh", f64::atanh, 1);
    // log
    pub const LN: Function = xprs_function!("ln", f64::ln, 1);
    pub const LOG: Function = xprs_function!("log", f64::log10, 1);
    // roots
    pub const SQRT: Function = xprs_function!("sqrt", f64::sqrt, 1);
    pub const CBRT: Function = xprs_function!("cbrt", f64::cbrt, 1);
    // misc
    pub const EXP: Function = xprs_function!("exp", f64::exp, 1);
    pub const ABS: Function = xprs_function!("abs", f64::abs, 1);
    pub const FLOOR: Function = xprs_function!("floor", f64::floor, 1);
    pub const CEIL: Function = xprs_function!("ceil", f64::ceil, 1);
    pub const ROUND: Function = xprs_function!("round", f64::round, 1);
    pub const TRUNC: Function = xprs_function!("trunc", f64::trunc, 1);
    pub const SUM: Function = xprs_function!("sum", |args| args.iter().sum());
}
