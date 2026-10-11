/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Operator};

/// Represents a binary operation in the abstract syntax tree (AST).
#[derive(Debug, PartialOrd, Clone)]
pub struct BinOp<'expr> {
    /// The operator of the binary operation.
    pub(crate) op: Operator,
    /// The left-hand side of the binary operation.
    pub(crate) lhs: Element<'expr>,
    /// The right-hand side of the binary operation.
    pub(crate) rhs: Element<'expr>,
}

impl<'expr> BinOp<'expr> {
    /// Creates a new [`BinOp`] from the binary operation components.
    const fn new(
        op: Operator,
        lhs: Element<'expr>,
        rhs: Element<'expr>,
    ) -> Self {
        Self { op, lhs, rhs }
    }

    /// Creates a new `Element::BinOp` from the binary operation components.
    pub(crate) fn new_element(
        op: Operator,
        lhs: Element<'expr>,
        rhs: Element<'expr>,
    ) -> Element<'expr> {
        Element::BinOp(Box::new(Self::new(op, lhs, rhs)))
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}

impl PartialEq for BinOp<'_> {
    #[expect(
        clippy::unreachable,
        reason = "factorial is never a binary operator"
    )]
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
                // not a binary operator
                Operator::Factorial => unreachable!(),
            }
    }
}
