/* Built-in imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::token::Operator;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct UnOp<'a> {
    pub(crate) op: Operator,
    pub(crate) operand: Element<'a>,
}

impl<'a> UnOp<'a> {
    pub const fn new(op: Operator, operand: Element<'a>) -> Self {
        Self { op, operand }
    }

    #[allow(clippy::unreachable)]
    pub fn simplify_for(self, var: (&str, f64)) -> Element<'a> {
        let operand = self.operand.simplify_for(var);
        if let Element::Number(num) = operand {
            match self.op {
                Operator::Plus => Element::Number(num),
                Operator::Minus => Element::Number(-num),
                Operator::Times | Operator::Divide | Operator::Power | Operator::Modulo => unreachable!(),
            }
        } else {
            Element::UnOp(Box::new(Self::new(self.op, operand)))
        }
    }
}

impl fmt::Display for UnOp<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({}{})", self.op, self.operand)
    }
}
