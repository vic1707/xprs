/* Built-in imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::token::Operator;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct BinOp<'a> {
    pub(crate) op: Operator,
    pub(crate) lhs: Element<'a>,
    pub(crate) rhs: Element<'a>,
}

impl<'a> BinOp<'a> {
    pub const fn new(op: Operator, lhs: Element<'a>, rhs: Element<'a>) -> Self {
        Self { op, lhs, rhs }
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}
