/* Crate imports */
use super::Element;
use crate::token::Operator;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UnOp<'a> {
    op: Operator,
    operand: Element<'a>,
}

impl<'a> UnOp<'a> {
    pub const PRECEDENCE: usize = 0;

    pub const fn new(op: Operator, operand: Element<'a>) -> Self {
        Self { op, operand }
    }
}
