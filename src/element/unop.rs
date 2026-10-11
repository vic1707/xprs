/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Operator};

/// Represents a unary operation in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct UnOp<'expr> {
    /// The operator of the unary operation.
    pub(crate) op: Operator,
    /// The operand of the unary operation.
    pub(crate) operand: Element<'expr>,
}

impl<'expr> UnOp<'expr> {
    /// Creates a new [`UnOp`] from the unary operation components.
    const fn new(op: Operator, operand: Element<'expr>) -> Self {
        Self { op, operand }
    }

    /// Creates a new `Element::UnOp` from the unary operation components.
    pub(crate) fn new_element(
        op: Operator,
        operand: Element<'expr>,
    ) -> Element<'expr> {
        Element::UnOp(Box::new(Self::new(op, operand)))
    }
}

impl fmt::Display for UnOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.op, self.operand)
    }
}
