/* Built-in imports */
use core::f64;
/* Crate imports */
use super::Function;
use crate::xprs_function;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Identifier<'a> {
    Function(Function<'a>),
    Constant(f64),
    Variable(&'a str),
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            /* Constants */
            "pi" => f64::consts::PI.into(),
            "e" => f64::consts::E.into(),
            /* Functions */
            // sin
            "sin" => xprs_function!(f64::sin, 1).into(),
            "sinh" => xprs_function!(f64::sinh, 1).into(),
            "asin" => xprs_function!(f64::asin, 1).into(),
            "asinh" => xprs_function!(f64::asinh, 1).into(),
            // cos
            "cos" => xprs_function!(f64::cos, 1).into(),
            "cosh" => xprs_function!(f64::cosh, 1).into(),
            "acos" => xprs_function!(f64::acos, 1).into(),
            "acosh" => xprs_function!(f64::acosh, 1).into(),
            // tan
            "tan" => xprs_function!(f64::tan, 1).into(),
            "tanh" => xprs_function!(f64::tanh, 1).into(),
            "atan" => xprs_function!(f64::atan, 1).into(),
            "atanh" => xprs_function!(f64::atanh, 1).into(),
            // log
            "ln" => xprs_function!(f64::ln, 1).into(),
            "log" => xprs_function!(f64::log10, 1).into(),
            // roots
            "sqrt" => xprs_function!(f64::sqrt, 1).into(),
            "cbrt" => xprs_function!(f64::cbrt, 1).into(),
            // misc
            "exp" => xprs_function!(f64::exp, 1).into(),
            "abs" => xprs_function!(f64::abs, 1).into(),
            "floor" => xprs_function!(f64::floor, 1).into(),
            "ceil" => xprs_function!(f64::ceil, 1).into(),
            "round" => xprs_function!(f64::round, 1).into(),
            "trunc" => xprs_function!(f64::trunc, 1).into(),
            "sum" => {
                Function::new_identifier("sum", |args| args.iter().sum(), None)
            },
            /* Variables */
            _ => Identifier::Variable(value),
        }
    }
}

impl From<f64> for Identifier<'_> {
    fn from(value: f64) -> Self {
        Identifier::Constant(value)
    }
}

impl<'a> From<Function<'a>> for Identifier<'a> {
    fn from(value: Function<'a>) -> Self {
        Identifier::Function(value)
    }
}
