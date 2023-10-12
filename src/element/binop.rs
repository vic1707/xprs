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

    pub fn simplify_for(self, var: (&str, f64)) -> Element<'a> {
        let lhs = self.lhs.simplify_for(var);
        let rhs = self.rhs.simplify_for(var);
        if let (&Element::Number(left), &Element::Number(right)) = (&lhs, &rhs) {
            Element::Number(match self.op {
                Operator::Plus => left + right,
                Operator::Minus => left - right,
                Operator::Times => left * right,
                Operator::Divide => left / right,
                Operator::Modulo => left % right,
                Operator::Power => left.powf(right),
            })
        } else {
            Element::BinOp(Box::new(Self::new(self.op, lhs, rhs)))
        }
    }
}

impl fmt::Display for BinOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}
