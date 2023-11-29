/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Operator};

/// Represents a unary operation in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd)]
pub struct UnOp<'a> {
    /// The operator of the unary operation.
    pub(crate) op: Operator,
    /// The operand of the unary operation.
    pub(crate) operand: Element<'a>,
}

impl<'a> UnOp<'a> {
    /// Creates a new [`UnOp`] from the unary operation components.
    const fn new(op: Operator, operand: Element<'a>) -> Self {
        Self { op, operand }
    }

    /// Creates a new `Element::UnOp` from the unary operation components.
    pub(crate) fn new_element(
        op: Operator,
        operand: Element<'a>,
    ) -> Element<'a> {
        Element::UnOp(Box::new(Self::new(op, operand)))
    }
}

impl fmt::Display for UnOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({}{})", self.op, self.operand)
    }
}
