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

    pub const fn precedence(op: &Operator) -> usize {
        match *op {
            Operator::Plus | Operator::Minus => 1,
            Operator::Times | Operator::Divide | Operator::Modulo => 2,
            Operator::Power => 3,
        }
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}
