/* Built-in imports */
use core::f64;
/* Crate imports */
use crate::yeet::yeet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Constant {
    Pi,
    Exponential,
}

impl TryFrom<&str> for Constant {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = match value {
            "pi" => Self::Pi,
            "e" => Self::Exponential,
            _ => yeet!("Invalid constant"),
        };

        Ok(val)
    }
}

impl Constant {
    pub const fn value(&self) -> f64 {
        match *self {
            Self::Pi => f64::consts::PI,
            Self::Exponential => f64::consts::E,
        }
    }
}
