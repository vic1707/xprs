/* Built-in imports */
use core::f64;
/* Crate imports */
use super::{function::built_in, Function};

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
            "sin" => built_in::SIN.into(),
            "sinh" => built_in::SINH.into(),
            "asin" => built_in::ASIN.into(),
            "asinh" => built_in::ASINH.into(),
            // cos
            "cos" => built_in::COS.into(),
            "cosh" => built_in::COSH.into(),
            "acos" => built_in::ACOS.into(),
            "acosh" => built_in::ACOSH.into(),
            // tan
            "tan" => built_in::TAN.into(),
            "tanh" => built_in::TANH.into(),
            "atan" => built_in::ATAN.into(),
            "atanh" => built_in::ATANH.into(),
            // log
            "ln" => built_in::LN.into(),
            "log" => built_in::LOG.into(),
            // roots
            "sqrt" => built_in::SQRT.into(),
            "cbrt" => built_in::CBRT.into(),
            // misc
            "exp" => built_in::EXP.into(),
            "abs" => built_in::ABS.into(),
            "floor" => built_in::FLOOR.into(),
            "ceil" => built_in::CEIL.into(),
            "round" => built_in::ROUND.into(),
            "trunc" => built_in::TRUNC.into(),
            "sum" => built_in::SUM.into(),
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
