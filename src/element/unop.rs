/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Operator};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UnOp<'a> {
    pub(crate) op: Operator,
    pub(crate) operand: Element<'a>,
}

impl<'a> UnOp<'a> {
    const fn new(op: Operator, operand: Element<'a>) -> Self {
        Self { op, operand }
    }

    pub fn new_element(op: Operator, operand: Element<'a>) -> Element<'a> {
        Element::UnOp(Box::new(Self::new(op, operand)))
    }
}

impl fmt::Display for UnOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({}{})", self.op, self.operand)
    }
}
