use super::{BinOp, Element, FunctionCall, UnOp};
use crate::token::Operator;

pub trait CompTime<'a> {
    fn simplify(self) -> Element<'a>;
}

impl<'a> CompTime<'a> for Element<'a> {
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

impl<'a> CompTime<'a> for BinOp<'a> {
    #[inline]
    #[allow(clippy::too_many_lines)]
    fn simplify(self) -> Element<'a> {
        use Element::Number;
        use Operator::{Divide, Minus, Modulo, Plus, Power, Times};
        match self {
            /////////////////////////// Additions ///////////////////////////
            // 0 + .. => ..
            BinOp { op: Plus, lhs, rhs } if lhs == Number(0.0_f64) => {
                rhs.simplify()
            },
            // .. + 0 => ..
            BinOp { op: Plus, lhs, rhs } if rhs == Number(0.0_f64) => {
                lhs.simplify()
            },
            ////////////////////////// Subtractions /////////////////////////
            // 0 - .. => -..
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == Number(0.0_f64) => {
                UnOp::new(Operator::Minus, rhs.simplify()).simplify()
            },
            // .. - 0 => ..
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if rhs == Number(0.0_f64) => lhs.simplify(),
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
            } if lhs == Number(1.0_f64) => rhs.simplify(),
            // .. * 1 => ..
            BinOp {
                op: Times,
                lhs,
                rhs,
            } if rhs == Number(1.0_f64) => lhs.simplify(),
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
            } if rhs == Number(1.0_f64) => lhs.simplify(),
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
            } if rhs == Number(1.0_f64) => lhs.simplify(),
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
                    #[allow(clippy::modulo_arithmetic)]
                    Modulo => lhs % rhs,
                };
                Number(result)
            },
            BinOp { op, rhs, lhs } => Element::BinOp(Box::new(BinOp::new(
                op,
                lhs.simplify(),
                rhs.simplify(),
            ))),
        }
    }
}

impl<'a> CompTime<'a> for UnOp<'a> {
    #[inline]
    fn simplify(self) -> Element<'a> {
        let operand = self.operand.simplify();
        #[allow(clippy::unreachable)]
        match self.op {
            Operator::Plus => operand,
            Operator::Minus => match operand {
                Element::Number(num) => Element::Number(-num),
                Element::UnOp(unop) => match unop.op {
                    Operator::Plus => Element::UnOp(Box::new(UnOp::new(
                        Operator::Minus,
                        unop.operand,
                    ))),
                    Operator::Minus => unop.operand,
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => unreachable!(),
                },
                Element::BinOp(_)
                | Element::Function(_)
                | Element::Variable(_) => Element::UnOp(Box::new(UnOp {
                    op: self.op,
                    operand,
                })),
            },
            Operator::Times
            | Operator::Divide
            | Operator::Power
            | Operator::Modulo => unreachable!(),
        }
    }
}

impl<'a> CompTime<'a> for FunctionCall<'a> {
    #[inline]
    fn simplify(self) -> Element<'a> {
        let arg = self.arg.simplify();
        match arg {
            Element::Number(num) => Element::Number((self.func)(num)),
            Element::BinOp(_)
            | Element::UnOp(_)
            | Element::Function(_)
            | Element::Variable(_) => {
                Element::Function(Box::new(FunctionCall {
                    func: self.func,
                    arg,
                }))
            },
        }
    }
}
