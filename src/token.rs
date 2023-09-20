/* Clippy config */
#![allow(clippy::pub_use)]
/* Built-in imports */
use core::f64;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Identifier<'a> {
    Function(fn(f64) -> f64),
    Constant(f64),
    Variable(&'a str),
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Operator {
    // Factorial,
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    Modulo,
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            /* Constants */
            "pi" => Identifier::Constant(f64::consts::PI),
            "e" => Identifier::Constant(f64::consts::E),
            /* Functions */
            // sin
            "sin" => Identifier::Function(f64::sin),
            "sinh" => Identifier::Function(f64::sinh),
            "asin" => Identifier::Function(f64::asin),
            "asinh" => Identifier::Function(f64::asinh),
            // cos
            "cos" => Identifier::Function(f64::cos),
            "cosh" => Identifier::Function(f64::cosh),
            "acos" => Identifier::Function(f64::acos),
            "acosh" => Identifier::Function(f64::acosh),
            // tan
            "tan" => Identifier::Function(f64::tan),
            "tanh" => Identifier::Function(f64::tanh),
            "atan" => Identifier::Function(f64::atan),
            "atanh" => Identifier::Function(f64::atanh),
            // log
            "ln" => Identifier::Function(f64::ln),
            "log" => Identifier::Function(f64::log10),
            // roots
            "sqrt" => Identifier::Function(f64::sqrt),
            "cbrt" => Identifier::Function(f64::cbrt),
            // misc
            "exp" => Identifier::Function(f64::exp),
            "abs" => Identifier::Function(f64::abs),
            "floor" => Identifier::Function(f64::floor),
            "ceil" => Identifier::Function(f64::ceil),
            "round" => Identifier::Function(f64::round),
            "trunc" => Identifier::Function(f64::trunc),
            /* Variables */
            _ => Identifier::Variable(value),
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // b'!' => Ok(Self::Factorial),
            b'+' => Ok(Self::Plus),
            b'-' => Ok(Self::Minus),
            b'*' => Ok(Self::Times),
            b'/' => Ok(Self::Divide),
            b'^' => Ok(Self::Power),
            b'%' => Ok(Self::Modulo),
            _ => Err("Operator not found"),
        }
    }
}
