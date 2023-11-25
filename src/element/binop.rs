/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Operator};

/// Represents a binary operation in the abstract syntax tree (AST).
#[derive(Debug, PartialOrd)]
pub struct BinOp<'a> {
    pub(crate) op: Operator,
    pub(crate) lhs: Element<'a>,
    pub(crate) rhs: Element<'a>,
}

impl<'a> BinOp<'a> {
    /// Creates a new [`BinOp`] from the binary operation components.
    const fn new(op: Operator, lhs: Element<'a>, rhs: Element<'a>) -> Self {
        Self { op, lhs, rhs }
    }

    /// Creates a new `Element::BinOp` from the binary operation components.
    pub(crate) fn new_element(
        op: Operator,
        lhs: Element<'a>,
        rhs: Element<'a>,
    ) -> Element<'a> {
        Element::BinOp(Box::new(Self::new(op, lhs, rhs)))
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}

impl PartialEq for BinOp<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
            && match self.op {
                // non commutative operators
                Operator::Divide
                | Operator::Minus
                | Operator::Modulo
                | Operator::Power => {
                    self.lhs == other.lhs && self.rhs == other.rhs
                },
                // commutative operators
                Operator::Plus | Operator::Times => {
                    (self.lhs == other.lhs && self.rhs == other.rhs)
                        || (self.lhs == other.rhs && self.rhs == other.lhs)
                },
            }
    }
}
