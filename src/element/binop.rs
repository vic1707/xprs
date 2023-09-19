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
}
