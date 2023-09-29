/* Build-it imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::token::Operator;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BinOp<'a> {
    op: Operator,
    lhs: Element<'a>,
    rhs: Element<'a>,
}

impl<'a> BinOp<'a> {
    pub const fn new(op: Operator, lhs: Element<'a>, rhs: Element<'a>) -> Self {
        Self { op, lhs, rhs }
    }

    #[cfg(feature = "pejmdas")]
    pub const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize = 3;
    #[cfg(feature = "pemdas")]
    pub const IMPLICIT_MULTIPLICATION_PRECEDENCE: usize =
        Self::precedence(&Operator::Times);

    pub const fn precedence(op: &Operator) -> usize {
        match *op {
            Operator::Plus | Operator::Minus => 1,
            Operator::Times | Operator::Divide | Operator::Modulo => 2,
            // uses `4` because `pejmdas` feature uses `3`
            Operator::Power => 4,
        }
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}
