/* Built-in imports */
use core::f64;
/* Crate imports */
use super::{function::function, Function};

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
            "pi" => Identifier::Constant(f64::consts::PI),
            "e" => Identifier::Constant(f64::consts::E),
            /* Functions */
            // sin
            "sin" => Identifier::Function(function!(f64::sin, 1)),
            "sinh" => Identifier::Function(function!(f64::sinh, 1)),
            "asin" => Identifier::Function(function!(f64::asin, 1)),
            "asinh" => Identifier::Function(function!(f64::asinh, 1)),
            // cos
            "cos" => Identifier::Function(function!(f64::cos, 1)),
            "cosh" => Identifier::Function(function!(f64::cosh, 1)),
            "acos" => Identifier::Function(function!(f64::acos, 1)),
            "acosh" => Identifier::Function(function!(f64::acosh, 1)),
            // tan
            "tan" => Identifier::Function(function!(f64::tan, 1)),
            "tanh" => Identifier::Function(function!(f64::tanh, 1)),
            "atan" => Identifier::Function(function!(f64::atan, 1)),
            "atanh" => Identifier::Function(function!(f64::atanh, 1)),
            // log
            "ln" => Identifier::Function(function!(f64::ln, 1)),
            "log" => Identifier::Function(function!(f64::log10, 1)),
            // roots
            "sqrt" => Identifier::Function(function!(f64::sqrt, 1)),
            "cbrt" => Identifier::Function(function!(f64::cbrt, 1)),
            // misc
            "exp" => Identifier::Function(function!(f64::exp, 1)),
            "abs" => Identifier::Function(function!(f64::abs, 1)),
            "floor" => Identifier::Function(function!(f64::floor, 1)),
            "ceil" => Identifier::Function(function!(f64::ceil, 1)),
            "round" => Identifier::Function(function!(f64::round, 1)),
            "trunc" => Identifier::Function(function!(f64::trunc, 1)),
            /* Variables */
            _ => Identifier::Variable(value),
        }
    }
}
