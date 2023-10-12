/* Built-in imports */
use core::fmt;
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::{element::Element, macros::yeet, token::Operator};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct Xprs<'a> {
    pub root: Element<'a>,
    pub vars: HashSet<&'a str>,
}

impl fmt::Display for Xprs<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.root)
    }
}

impl Xprs<'_> {
    #[inline]
    pub fn eval(
        &self,
        variables: &HashMap<&str, f64>,
    ) -> Result<f64, &'static str> {
        // check if all variables are present
        for var in &self.vars {
            if !variables.contains_key(var) {
                yeet!("Variable not found");
            }
        }

        XprsImpl::new(variables).eval_element(&self.root)
    }

    #[inline]
    #[must_use]
    pub fn simplify_for(self, var: (&str, f64)) -> Self {
        let root = self.root.simplify_for(var);
        let mut vars = self.vars;
        vars.remove(var.0);
        Self { root, vars }
    }
}

struct XprsImpl<'a> {
    variables: &'a HashMap<&'a str, f64>,
}

impl XprsImpl<'_> {
    #[inline]
    pub const fn new<'a>(variables: &'a HashMap<&'a str, f64>) -> XprsImpl<'a> {
        XprsImpl { variables }
    }

    #[allow(clippy::ref_patterns)]
    fn eval_element(&self, element: &Element) -> Result<f64, &'static str> {
        let res = match *element {
            Element::Number(n) => n,
            Element::Variable(name) => {
                *self.variables.get(name).ok_or("Variable not found")?
            },
            Element::UnOp(ref unop) => {
                let operand = self.eval_element(&unop.operand)?;
                match unop.op {
                    Operator::Plus => operand,
                    Operator::Minus => -operand,
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => yeet!("Invalid unary operator"),
                }
            },
            Element::BinOp(ref binop) => {
                let left = self.eval_element(&binop.lhs)?;
                let right = self.eval_element(&binop.rhs)?;
                match binop.op {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Times => left * right,
                    Operator::Divide => left / right,
                    Operator::Power => left.powf(right),
                    Operator::Modulo => left % right,
                }
            },
            Element::Function(ref func) => {
                let arg = self.eval_element(&func.arg)?;
                (func.func)(arg)
            },
        };

        Ok(res)
    }
}
