use super::{BinOp, Element, FunctionCall, UnOp};
use crate::token::Operator;

pub trait Simplify<'a> {
    fn simplify_for(self, var: (&str, f64)) -> Element<'a>;
    fn simplify(self) -> Element<'a>;
}

impl<'a> Simplify<'a> for Element<'a> {
    #[inline]
    fn simplify_for(self, var: (&str, f64)) -> Self {
        match self {
            Self::BinOp(binop) => binop.simplify_for(var),
            Self::UnOp(unop) => unop.simplify_for(var),
            Self::Function(func) => func.simplify_for(var),
            Self::Variable(name) if name == var.0 => Self::Number(var.1),
            Self::Number(_) | Self::Variable(_) => self,
        }
    }

    #[inline]
    fn simplify(self) -> Self {
        match self {
            Self::BinOp(binop) => binop.simplify(),
            Self::UnOp(unop) => unop.simplify(),
            Self::Function(func) => func.simplify(),
            Self::Number(_) | Self::Variable(_) => self,
        }
    }
}

impl<'a> Simplify<'a> for BinOp<'a> {
    #[inline]
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.lhs = self.lhs.simplify_for(var);
        self.rhs = self.rhs.simplify_for(var);
        self.simplify()
    }

    #[inline]
    #[allow(clippy::too_many_lines)]
    fn simplify(mut self) -> Element<'a> {
        use Element::Number;
        use Operator::{Divide, Minus, Modulo, Plus, Power, Times};
        self.lhs = self.lhs.simplify();
        self.rhs = self.rhs.simplify();
        match self {
            /////////////////////////// Additions ///////////////////////////
            // 0 + .. => ..
            BinOp { op: Plus, lhs, rhs } if lhs == Number(0.0_f64) => rhs,
            // .. + 0 => ..
            BinOp { op: Plus, lhs, rhs } if rhs == Number(0.0_f64) => lhs,
            ////////////////////////// Subtractions /////////////////////////
            // 0 - .. => -..
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == Number(0.0_f64) => {
                UnOp::new(Operator::Minus, rhs).simplify()
            },
            // .. - 0 => ..
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if rhs == Number(0.0_f64) => lhs,
            // .. - .. => 0
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == rhs => Number(0.0_f64),
            //////////////////////// Multiplications ////////////////////////
            // 0 * .. => 0
            BinOp { op: Times, lhs, .. } if lhs == Number(0.0_f64) => {
                Number(0.0_f64)
            },
            // .. * 0 => 0
            BinOp { op: Times, rhs, .. } if rhs == Number(0.0_f64) => {
                Number(0.0_f64)
            },
            // 1 * .. => ..
            BinOp {
                op: Times,
                lhs,
                rhs,
            } if lhs == Number(1.0_f64) => rhs,
            // .. * 1 => ..
            BinOp {
                op: Times,
                lhs,
                rhs,
            } if rhs == Number(1.0_f64) => lhs,
            /////////////////////////// Divisions ///////////////////////////
            // 0 / .. => 0
            BinOp {
                op: Divide, lhs, ..
            } if lhs == Number(0.0_f64) => Number(0.0_f64),
            // .. / 0 => inf
            BinOp {
                op: Divide, rhs, ..
            } if rhs == Number(0.0_f64) => Number(f64::INFINITY),
            // .. / 1 => ..
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if rhs == Number(1.0_f64) => lhs,
            // .. / .. => 1
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if lhs == rhs => Number(1.0_f64),
            ///////////////////////////// Powers ////////////////////////////
            // 0 ^ .. => 0
            BinOp { op: Power, lhs, .. } if lhs == Number(0.0_f64) => {
                Number(0.0_f64)
            },
            // .. ^ 0 => 1
            BinOp {
                op: Divide, rhs, ..
            } if rhs == Number(0.0_f64) => Number(1.0_f64),
            // .. ^ 1 => ..
            BinOp {
                op: Power,
                lhs,
                rhs,
            } if rhs == Number(1.0_f64) => lhs,
            //////////////////////////// Modulos ////////////////////////////
            // 0 % .. => 0
            BinOp {
                op: Modulo, lhs, ..
            } if lhs == Number(0.0_f64) => Number(0.0_f64),
            // .. % 0 => NaN
            BinOp {
                op: Modulo, rhs, ..
            } if rhs == Number(0.0_f64) => Number(f64::NAN),
            // .. % 1 => 0
            BinOp {
                op: Modulo, rhs, ..
            } if rhs == Number(1.0_f64) => Number(0.0_f64),
            // .. % .. => 0
            BinOp {
                op: Modulo,
                lhs,
                rhs,
            } if lhs == rhs => Number(0.0_f64),
            // other
            BinOp {
                op,
                rhs: Number(rhs),
                lhs: Number(lhs),
            } => {
                let result = match op {
                    Plus => lhs + rhs,
                    Minus => lhs - rhs,
                    Times => lhs * rhs,
                    Divide => lhs / rhs,
                    Power => lhs.powf(rhs),
                    Modulo => lhs % rhs,
                };
                Number(result)
            },
            _ => self.into(),
        }
    }
}

impl<'a> Simplify<'a> for UnOp<'a> {
    #[inline]
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.operand = self.operand.simplify_for(var);
        self.simplify()
    }

    #[inline]
    fn simplify(mut self) -> Element<'a> {
        self.operand = self.operand.simplify();
        #[allow(clippy::unreachable)]
        match self.op {
            Operator::Plus => self.operand,
            Operator::Minus => match self.operand {
                Element::Number(num) => Element::Number(-num),
                Element::UnOp(unop) => match unop.op {
                    Operator::Plus => {
                        UnOp::new_element(Operator::Minus, unop.operand)
                    },
                    Operator::Minus => unop.operand,
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => unreachable!(),
                },
                Element::BinOp(_)
                | Element::Function(_)
                | Element::Variable(_) => self.into(),
            },
            Operator::Times
            | Operator::Divide
            | Operator::Power
            | Operator::Modulo => unreachable!(),
        }
    }
}

impl<'a> Simplify<'a> for FunctionCall<'a> {
    #[inline]
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.arg = self.arg.simplify_for(var);
        self.simplify()
    }

    #[inline]
    fn simplify(mut self) -> Element<'a> {
        self.arg = self.arg.simplify();
        match self.arg {
            Element::Number(num) => Element::Number((self.func)(num)),
            Element::BinOp(_)
            | Element::UnOp(_)
            | Element::Function(_)
            | Element::Variable(_) => self.into(),
        }
    }
}
