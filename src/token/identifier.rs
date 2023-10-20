/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::misc::Function;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Identifier<'a> {
    Function(&'a str, Function),
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
            "sin" => Identifier::Function(value, f64::sin),
            "sinh" => Identifier::Function(value, f64::sinh),
            "asin" => Identifier::Function(value, f64::asin),
            "asinh" => Identifier::Function(value, f64::asinh),
            // cos
            "cos" => Identifier::Function(value, f64::cos),
            "cosh" => Identifier::Function(value, f64::cosh),
            "acos" => Identifier::Function(value, f64::acos),
            "acosh" => Identifier::Function(value, f64::acosh),
            // tan
            "tan" => Identifier::Function(value, f64::tan),
            "tanh" => Identifier::Function(value, f64::tanh),
            "atan" => Identifier::Function(value, f64::atan),
            "atanh" => Identifier::Function(value, f64::atanh),
            // log
            "ln" => Identifier::Function(value, f64::ln),
            "log" => Identifier::Function(value, f64::log10),
            // roots
            "sqrt" => Identifier::Function(value, f64::sqrt),
            "cbrt" => Identifier::Function(value, f64::cbrt),
            // misc
            "exp" => Identifier::Function(value, f64::exp),
            "abs" => Identifier::Function(value, f64::abs),
            "floor" => Identifier::Function(value, f64::floor),
            "ceil" => Identifier::Function(value, f64::ceil),
            "round" => Identifier::Function(value, f64::round),
            "trunc" => Identifier::Function(value, f64::trunc),
            /* Variables */
            _ => Identifier::Variable(value),
        }
    }
}
