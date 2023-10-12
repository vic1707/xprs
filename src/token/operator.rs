/* Built-in imports */
use core::fmt;

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

impl fmt::Display for Operator {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // Self::Factorial => write!(f, "!"),
            Self::Plus => write!(fmt, "+"),
            Self::Minus => write!(fmt, "-"),
            Self::Times => write!(fmt, "*"),
            Self::Divide => write!(fmt, "/"),
            Self::Power => write!(fmt, "^"),
            Self::Modulo => write!(fmt, "%"),
        }
    }
}
