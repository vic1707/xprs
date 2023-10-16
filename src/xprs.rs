/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::{fmt, ptr};
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::{element::Element, macros::trust_me, token::Operator};

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
    ) -> Result<f64, EvalError> {
        XprsImpl::new(variables).eval_element(&self.root)
    }

    #[inline]
    pub fn simplify_for_inplace(&mut self, var: (&str, f64)) -> bool {
        let mut tmp = trust_me!(ptr::read(&self.root));
        tmp = tmp.simplify_for(var);
        trust_me!(ptr::write(&mut self.root, tmp););
        self.vars.remove(var.0)
    }

    #[inline]
    #[must_use]
    pub fn simplify_for(mut self, var: (&str, f64)) -> Self {
        self.simplify_for_inplace(var);
        self
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

    #[allow(clippy::ref_patterns, clippy::unreachable)]
    fn eval_element(&self, element: &Element) -> Result<f64, EvalError> {
        let res = match *element {
            Element::Number(n) => n,
            Element::Variable(name) => *self
                .variables
                .get(name)
                .ok_or_else(|| EvalError(name.to_owned()))?,
            Element::UnOp(ref unop) => {
                let operand = self.eval_element(&unop.operand)?;
                match unop.op {
                    Operator::Plus => operand,
                    Operator::Minus => -operand,
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => unreachable!(),
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

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("Evaluation error: '{0}' was not provided")]
pub struct EvalError(String);

//////////////////////////////////////////////////////////////////////////////
//  TODO: replace this with variadic generics when it's available & stable  //
//////////////////////////////////////////////////////////////////////////////
#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
impl<'a> Xprs<'a> {
    #[inline]
    pub fn bind(self, var: &'a str) -> impl Fn(f64) -> Result<f64, EvalError> + 'a {
        move |val| self.eval(&[(var, val)].into())
    }

    #[inline]
    pub fn bind2(self, var1: &'a str, var2: &'a str) -> impl Fn(f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2| self.eval(&[(var1, val1), (var2, val2)].into())
    }

    #[inline]
    pub fn bind3(self, var1: &'a str, var2: &'a str, var3: &'a str) -> impl Fn(f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3| self.eval(&[(var1, val1), (var2, val2), (var3, val3)].into())
    }

    #[inline]
    pub fn bind4(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str) -> impl Fn(f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4)].into())
    }

    #[inline]
    pub fn bind5(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str) -> impl Fn(f64, f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4, val5| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5)].into())
    }

    #[inline]
    pub fn bind6(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str) -> impl Fn(f64, f64, f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4, val5, val6| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6)].into())
    }

    #[inline]
    pub fn bind7(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str) -> impl Fn(f64, f64, f64, f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4, val5, val6, val7| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7)].into())
    }

    #[inline]
    pub fn bind8(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str, var8: &'a str) -> impl Fn(f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4, val5, val6, val7, val8| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7), (var8, val8)].into())
    }

    #[inline]
    pub fn bind9(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str, var8: &'a str, var9: &'a str) -> impl Fn(f64, f64, f64, f64, f64, f64, f64, f64, f64) -> Result<f64, EvalError> + 'a {
        move |val1, val2, val3, val4, val5, val6, val7, val8, val9| self.eval(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7), (var8, val8), (var9, val9)].into())
    }

    // NOTE: Too lazy to implement this for more than 9 variables even with Copilot
    // + I don't really think anyone will need more than 9 variables anyway
    #[inline]
    pub fn bind_n<const T: usize>(self, vars: [&'a str; T]) -> impl Fn([f64; T]) -> Result<f64, EvalError> + 'a {
        move |vals| self.eval(&vars.iter().copied().zip(vals).collect())
    }
    #[inline]
    pub fn bind_n_runtime(self, vars: &'a [&'a str]) -> impl Fn(&[f64]) -> Result<f64, EvalError> + 'a {
        move |vals| self.eval(&vars.iter().copied().zip(vals.iter().copied()).collect())
    }
}
