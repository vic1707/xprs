/* Built-in imports */
use core::f64;
/* Crate imports */
use super::Function;
use crate::utils::built_in_functions;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Identifier<'a> {
    Function(&'a Function<'a>),
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
            "sin" => (&built_in_functions::SIN).into(),
            "sinh" => (&built_in_functions::SINH).into(),
            "asin" => (&built_in_functions::ASIN).into(),
            "asinh" => (&built_in_functions::ASINH).into(),
            // cos
            "cos" => (&built_in_functions::COS).into(),
            "cosh" => (&built_in_functions::COSH).into(),
            "acos" => (&built_in_functions::ACOS).into(),
            "acosh" => (&built_in_functions::ACOSH).into(),
            // tan
            "tan" => (&built_in_functions::TAN).into(),
            "tanh" => (&built_in_functions::TANH).into(),
            "atan" => (&built_in_functions::ATAN).into(),
            "atan2" => (&built_in_functions::ATAN2).into(),
            "atanh" => (&built_in_functions::ATANH).into(),
            // log
            "ln" => (&built_in_functions::LN).into(),
            "log" => (&built_in_functions::LOG).into(),
            "logn" => (&built_in_functions::LOGN).into(),
            // roots
            "sqrt" => (&built_in_functions::SQRT).into(),
            "cbrt" => (&built_in_functions::CBRT).into(),
            // misc
            "exp" => (&built_in_functions::EXP).into(),
            "abs" => (&built_in_functions::ABS).into(),
            "floor" => (&built_in_functions::FLOOR).into(),
            "ceil" => (&built_in_functions::CEIL).into(),
            "round" => (&built_in_functions::ROUND).into(),
            "trunc" => (&built_in_functions::TRUNC).into(),
            "sum" => (&built_in_functions::SUM).into(),
            "mean" => (&built_in_functions::MEAN).into(),
            "invert" => (&built_in_functions::INVERT).into(),
            "min" => (&built_in_functions::MIN).into(),
            "max" => (&built_in_functions::MAX).into(),
            "hypot" => (&built_in_functions::HYPOT).into(),
            "fract" => (&built_in_functions::FRACT).into(),
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

impl<'a> From<&'a Function<'a>> for Identifier<'a> {
    fn from(value: &'a Function<'a>) -> Self {
        Identifier::Function(value)
    }
}
