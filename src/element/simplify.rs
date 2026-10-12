/* Crate imports */
use crate::{
    element::{BinOp, Element, FunctionCall, UnOp},
    token::Operator,
    utils::factorial::factorial,
};

/// Trait for simplifying abstract syntax tree (AST) elements.
/// Is enabled with the `compile-time-optimizations` feature (on by default).
pub trait Simplify<'expr> {
    /// Simplifies the element for the specified variable.
    fn simplify_for(self, var: (&str, f64)) -> Element<'expr>;

    /// Simplifies the element.
    fn simplify(self) -> Element<'expr>;
}

impl<'expr> Simplify<'expr> for Element<'expr> {
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

impl<'expr> Simplify<'expr> for BinOp<'expr> {
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'expr> {
        self.lhs = self.lhs.simplify_for(var);
        self.rhs = self.rhs.simplify_for(var);
        self.simplify()
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep algebraic simplification rules in one match"
    )]
    fn simplify(mut self) -> Element<'expr> {
        use Element::Number;
        use Operator::{Divide, Factorial, Minus, Modulo, Plus, Power, Times};
        self.lhs = self.lhs.simplify();
        self.rhs = self.rhs.simplify();
        match self {
            /////////////////////////// Additions ///////////////////////////
            // 0 + a => a
            BinOp {
                op: Plus,
                lhs: Number(0.0),
                rhs,
            } => rhs,
            // a + 0 => a
            BinOp {
                op: Plus,
                lhs,
                rhs: Number(0.0),
            } => lhs,
            // (-a) + a => 0
            BinOp {
                op: Plus,
                lhs: Element::UnOp(unop),
                rhs,
            } if unop.op == Minus && unop.operand == rhs => Number(0.0),
            // a + (-a) => 0
            BinOp {
                op: Plus,
                lhs,
                rhs: Element::UnOp(unop),
            } if unop.op == Minus && lhs == unop.operand => Number(0.0),
            ////////////////////////// Subtractions /////////////////////////
            // 0 - a => -a
            BinOp {
                op: Minus,
                lhs: Number(0.0),
                rhs,
            } => UnOp::new_element(Operator::Minus, rhs).simplify(),
            // a - 0 => a
            BinOp {
                op: Minus,
                lhs,
                rhs: Number(0.0),
            } => lhs,
            // a - a => 0
            BinOp {
                op: Minus,
                lhs,
                rhs,
            } if lhs == rhs => Number(0.0),
            // a - (-b) => a + b
            BinOp {
                op: Minus,
                lhs,
                rhs: Element::UnOp(unop),
            } if unop.op == Minus => {
                BinOp::new_element(Operator::Plus, lhs, unop.operand).simplify()
            },
            //////////////////////// Multiplications ////////////////////////
            // 0 * a and a * 0 => 0
            BinOp {
                op: Times,
                lhs: Number(0.0),
                ..
            }
            | BinOp {
                op: Times,
                rhs: Number(0.0),
                ..
            } => Number(0.0),
            // 1 * a => a
            BinOp {
                op: Times,
                lhs: Number(1.0),
                rhs,
            } => rhs,
            // a * 1, a / 1 and a ^ 1 => a
            BinOp {
                op: Times | Divide | Power,
                lhs,
                rhs: Number(1.0),
            } => lhs,
            // (-a) * (-b) => a * b
            BinOp {
                op: Times,
                lhs: Element::UnOp(lhs),
                rhs: Element::UnOp(rhs),
            } if lhs.op == Minus && rhs.op == Minus => {
                BinOp::new_element(Operator::Times, lhs.operand, rhs.operand)
                    .simplify()
            },
            /////////////////////////// Divisions ///////////////////////////
            // 0/0 and 0%0 => NaN // special cases
            BinOp {
                op: Divide | Modulo,
                lhs: Number(0.0),
                rhs: Number(0.0),
            } => Number(f64::NAN),
            // 0 / a and 0 % a => 0
            BinOp {
                op: Divide | Modulo,
                lhs: Number(0.0),
                ..
            } => Number(0.0),
            // a / 0 => inf
            BinOp {
                op: Divide,
                rhs: Number(0.0),
                ..
            } => Number(f64::INFINITY),
            // a / a => 1
            BinOp {
                op: Divide,
                lhs,
                rhs,
            } if lhs == rhs => Number(1.0),
            // (-a) / (-b) => a / b
            BinOp {
                op: Divide,
                lhs: Element::UnOp(lhs),
                rhs: Element::UnOp(rhs),
            } if lhs.op == Minus && rhs.op == Minus => {
                BinOp::new_element(Operator::Divide, lhs.operand, rhs.operand)
                    .simplify()
            },
            ///////////////////////////// Powers ////////////////////////////
            // 0 ^ 0 => 1 // special case
            BinOp {
                op: Power,
                lhs: Number(0.0),
                rhs: Number(0.0),
            } => Number(1.0),
            // 0 ^ a => 0
            BinOp {
                op: Power,
                lhs: Number(0.0),
                ..
            } => Number(0.0),
            // a ^ 0 => 1
            BinOp {
                op: Power,
                rhs: Number(0.0),
                ..
            } => Number(1.0),
            //////////////////////////// Modulos ////////////////////////////
            // a % 0 => NaN
            BinOp {
                op: Modulo,
                rhs: Number(0.0),
                ..
            } => Number(f64::NAN),
            // a % 1 => 0
            BinOp {
                op: Modulo,
                rhs: Number(1.0),
                ..
            } => Number(0.0),
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
                #[expect(
                    clippy::unreachable,
                    reason = "factorial is never a binary operator"
                )]
                let result = match op {
                    Plus => lhs + rhs,
                    Minus => lhs - rhs,
                    Times => lhs * rhs,
                    Divide => lhs / rhs,
                    Power => lhs.powf(rhs),
                    Modulo => lhs % rhs,
                    Factorial => unreachable!(),
                };
                Number(result)
            },
            _ => self.into(),
        }
    }
}

impl<'expr> Simplify<'expr> for UnOp<'expr> {
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'expr> {
        self.operand = self.operand.simplify_for(var);
        self.simplify()
    }

    fn simplify(mut self) -> Element<'expr> {
        self.operand = self.operand.simplify();
        #[expect(
            clippy::unreachable,
            reason = "binary operators never appear in a unary expression"
        )]
        match self.op {
            Operator::Plus => self.operand,
            Operator::Factorial => match self.operand {
                Element::Number(num) => Element::Number(factorial(num)),
                Element::UnOp(_)
                | Element::BinOp(_)
                | Element::Function(_)
                | Element::Variable(_) => self.into(),
            },
            Operator::Minus => match self.operand {
                Element::Number(num) => Element::Number(-num),
                // -(-a) => a, including signed zero and NaN.
                Element::UnOp(unop) if unop.op == Operator::Minus => {
                    unop.operand
                },
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

impl<'expr> Simplify<'expr> for FunctionCall<'expr> {
    fn simplify_for(mut self, var: (&str, f64)) -> Element<'expr> {
        self.args = self
            .args
            .into_iter()
            .map(|arg| arg.simplify_for(var))
            .collect();
        self.simplify()
    }

    fn simplify(mut self) -> Element<'expr> {
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
