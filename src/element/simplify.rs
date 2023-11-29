/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    token::Operator,
};

/// Trait for simplifying abstract syntax tree (AST) elements.
/// Is enabled with the `compile-time-optimizations` feature (on by default).
pub trait Simplify<'a> {
    /// Simplifies the element for the specified variable.
    fn simplify_for(self, var: (&str, f64)) -> Element<'a>;

    /// Simplifies the element.
    fn simplify(self) -> Element<'a>;
}

impl<'a> Simplify<'a> for Element<'a> {
    fn simplify_for(self, var: (&str, f64)) -> Self {
        match self {
            Self::BinOp(binop) => binop.simplify_for(var),
            Self::UnOp(unop) => unop.simplify_for(var),
            Self::Function(func) => func.simplify_for(var),
            Self::Variable(name) if name == var.0 => Self::Number(var.1),
            Self::Number(_) | Self::Variable(_) => self,
        }
    }

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
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.lhs = self.lhs.simplify_for(var);
        self.rhs = self.rhs.simplify_for(var);
        self.simplify()
    }

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn simplify(mut self) -> Element<'a> {
        use Element::Number;
        use Operator::{Divide, Minus, Modulo, Plus, Power, Times};
        self.lhs = self.lhs.simplify();
        self.rhs = self.rhs.simplify();
        match self {
            /////////////////////////// Additions ///////////////////////////
            // 0 + a => a
            BinOp { op: Plus, lhs, rhs } if lhs == Number(0.0) => rhs,
            // a + 0 => a
            BinOp { op: Plus, lhs, rhs } if rhs == Number(0.0) => lhs,
            ////// NIGHTLY FEATURES //////
            #[cfg(NIGHTLY)]
            // (-a) + a => 0
            BinOp {
                op: Plus,
                lhs: Element::UnOp(box UnOp { op: Minus, operand }),
                rhs,
            } if operand == rhs => Number(0.0),
            #[cfg(NIGHTLY)]
            // a + (-a) => 0
            BinOp {
                op: Plus,
                lhs,
                rhs: Element::UnOp(box UnOp { op: Minus, operand }),
            } if lhs == operand => Number(0.0),
            ////////////////////////// Subtractions /////////////////////////
            // 0 - a => -a
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == Number(0.0) => UnOp::new_element(Operator::Minus, rhs),
            // a - 0 => a
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if rhs == Number(0.0) => lhs,
            // a - a => 0
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == rhs => Number(0.0),
            ////// NIGHTLY FEATURES //////
            #[cfg(NIGHTLY)]
            // a - (-b) => a + b
            BinOp {
                op: Minus,
                lhs,
                rhs: Element::UnOp(box UnOp { op: Minus, operand }),
            } => BinOp::new_element(Operator::Plus, lhs, operand),
            //////////////////////// Multiplications ////////////////////////
            // 0 * a => 0
            BinOp { op: Times, lhs, .. } if lhs == Number(0.0) => Number(0.0),
            // a * 0 => 0
            BinOp { op: Times, rhs, .. } if rhs == Number(0.0) => Number(0.0),
            // 1 * a => a
            BinOp {
                op: Times,
                lhs,
                rhs,
            } if lhs == Number(1.0) => rhs,
            // a * 1 => a
            BinOp {
                op: Times,
                lhs,
                rhs,
            } if rhs == Number(1.0) => lhs,
            ////// NIGHTLY FEATURES //////
            // (-a) * (-b) => a * b
            #[cfg(NIGHTLY)]
            BinOp {
                op: Times,
                lhs:
                    Element::UnOp(box UnOp {
                        op: Minus,
                        operand: lhs,
                    }),
                rhs:
                    Element::UnOp(box UnOp {
                        op: Minus,
                        operand: rhs,
                    }),
            } => BinOp::new_element(Operator::Times, lhs, rhs),
            /////////////////////////// Divisions ///////////////////////////
            // 0/0 => NaN // special case
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if lhs == Number(0.0) && rhs == Number(0.0) => Number(f64::NAN),
            // 0 / a => 0
            BinOp {
                op: Divide, lhs, ..
            } if lhs == Number(0.0) => Number(0.0),
            // a / 0 => inf
            BinOp {
                op: Divide, rhs, ..
            } if rhs == Number(0.0) => Number(f64::INFINITY),
            // a / 1 => a
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if rhs == Number(1.0) => lhs,
            // a / a => 1
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if lhs == rhs => Number(1.0),
            ////// NIGHTLY FEATURES //////
            // (-a) / (-b) => a / b
            #[cfg(NIGHTLY)]
            BinOp {
                op: Divide,
                lhs:
                    Element::UnOp(box UnOp {
                        op: Minus,
                        operand: lhs,
                    }),
                rhs:
                    Element::UnOp(box UnOp {
                        op: Minus,
                        operand: rhs,
                    }),
            } => BinOp::new_element(Operator::Divide, lhs, rhs),
            ///////////////////////////// Powers ////////////////////////////
            // 0 ^ 0 => 1 // special case
            BinOp {
                op: Power,
                lhs,
                rhs,
            } if lhs == Number(0.0) && rhs == Number(0.0) => Number(1.0),
            // 0 ^ a => 0
            BinOp { op: Power, lhs, .. } if lhs == Number(0.0) => Number(0.0),
            // a ^ 0 => 1
            BinOp {
                op: Divide, rhs, ..
            } if rhs == Number(0.0) => Number(1.0),
            // a ^ 1 => a
            BinOp {
                op: Power,
                lhs,
                rhs,
            } if rhs == Number(1.0) => lhs,
            //////////////////////////// Modulos ////////////////////////////
            // 0 % 0 => NaN // special case
            BinOp {
                op: Modulo,
                lhs,
                rhs,
            } if lhs == Number(0.0) && rhs == Number(0.0) => Number(f64::NAN),
            // 0 % a => 0
            BinOp {
                op: Modulo, lhs, ..
            } if lhs == Number(0.0) => Number(0.0),
            // a % 0 => NaN
            BinOp {
                op: Modulo, rhs, ..
            } if rhs == Number(0.0) => Number(f64::NAN),
            // a % 1 => 0
            BinOp {
                op: Modulo, rhs, ..
            } if rhs == Number(1.0) => Number(0.0),
            // a % a => 0
            BinOp {
                op: Modulo,
                lhs,
                rhs,
            } if lhs == rhs => Number(0.0),
            /////////////////////////// 2 Numbers ///////////////////////////
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
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.operand = self.operand.simplify_for(var);
        self.simplify()
    }

    fn simplify(mut self) -> Element<'a> {
        self.operand = self.operand.simplify();
        #[allow(clippy::unreachable)]
        match self.op {
            Operator::Plus => self.operand,
            Operator::Minus => match self.operand {
                Element::Number(num) => Element::Number(-num),
                Element::UnOp(_)
                | Element::BinOp(_)
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
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'a> {
        self.args = self
            .args
            .into_iter()
            .map(|arg| arg.simplify_for(var))
            .collect();
        self.simplify()
    }

    fn simplify(mut self) -> Element<'a> {
        // TODO: Not a big fan of the second vector.
        // We need to simplify the arguments in all cases, but
        // if they are all numbers, we can call the function.
        let mut args_values: Vec<f64> = Vec::with_capacity(self.args.len());

        self.args = self
            .args
            .into_iter()
            .map(|arg| {
                let simplified = arg.simplify();
                if let Element::Number(num) = simplified {
                    args_values.push(num);
                }
                simplified
            })
            .collect();

        if args_values.len() == self.args.len() {
            self.call(&args_values).into()
        } else {
            self.into()
        }
    }
}
