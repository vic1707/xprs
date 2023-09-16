/* Crate imports */
use crate::yeet::yeet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function(fn(f64) -> f64);

impl TryFrom<&str> for Function {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let func = match value {
            "sin" => f64::sin,
            "cos" => f64::cos,
            "tan" => f64::tan,
            "asin" => f64::asin,
            "acos" => f64::acos,
            "atan" => f64::atan,
            "sinh" => f64::sinh,
            "cosh" => f64::cosh,
            "tanh" => f64::tanh,
            "asinh" => f64::asinh,
            "acosh" => f64::acosh,
            "atanh" => f64::atanh,
            "log" => f64::log10,
            "ln" => f64::ln,
            "exp" => f64::exp,
            "sqrt" => f64::sqrt,
            "cbrt" => f64::cbrt,
            "abs" => f64::abs,
            _ => yeet!("Invalid function"),
        };

        Ok(Self(func))
    }
}
