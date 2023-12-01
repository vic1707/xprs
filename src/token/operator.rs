/* Built-in imports */
use core::fmt;

/// Represents a mathematical operator.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
pub enum Operator {
    /// Addition operator.
    Plus,
    /// Subtraction operator.
    Minus,
    /// Multiplication operator.
    Times,
    /// Division operator.
    Divide,
    /// Exponentiation operator.
    Power,
    /// Modulo operator.
    Modulo,
    /// Factorial operator.
    Factorial,
}

impl TryFrom<u8> for Operator {
    type Error = &'static str;

    /// Attempts to convert a byte value into an [`Operator`].
    /// Returns an error if the byte value does not correspond to a valid operator.
    /// Valid operators are: '+', '-', '*', '/', '^', '%'.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'!' => Ok(Self::Factorial),
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
            Self::Factorial => write!(fmt, "!"),
            Self::Plus => write!(fmt, "+"),
            Self::Minus => write!(fmt, "-"),
            Self::Times => write!(fmt, "*"),
            Self::Divide => write!(fmt, "/"),
            Self::Power => write!(fmt, "^"),
            Self::Modulo => write!(fmt, "%"),
        }
    }
}
