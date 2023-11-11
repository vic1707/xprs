/* Clippy Config */
#![allow(clippy::std_instead_of_core)]
/* Built-in imports */
use core::{fmt, ptr};
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::{
    element::Element,
    element::Simplify,
    token::Operator,
    utils::macros::{trust_me, yeet},
};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct Xprs<'a> {
    pub root: Element<'a>,
    pub vars: HashSet<&'a str>,
}

impl<'input> TryFrom<&'input str> for Xprs<'input> {
    type Error = crate::ParseError;

    #[inline]
    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let parser = crate::Parser::default();
        let xprs = parser.parse(value);
        xprs
    }
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
    #[must_use]
    pub fn eval_unchecked(&self, variables: &HashMap<&str, f64>) -> f64 {
        XprsImpl::new(variables).eval_element_unchecked(&self.root)
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

    #[inline]
    pub fn simplify_for_multiple_inplace(&mut self, vars: &[(&str, f64)]) {
        // rewriting `simplify_for_inplace` to avoid dozens of `ptr::read` and `ptr::write`
        let mut tmp = trust_me!(ptr::read(&self.root));
        for &var in vars {
            tmp = tmp.simplify_for(var);
            self.vars.remove(var.0);
        }
        trust_me!(ptr::write(&mut self.root, tmp););
    }

    #[inline]
    #[must_use]
    pub fn simplify_for_multiple(mut self, vars: &[(&str, f64)]) -> Self {
        self.simplify_for_multiple_inplace(vars);
        self
    }
}

struct XprsImpl<'a> {
    variables: &'a HashMap<&'a str, f64>,
}

impl XprsImpl<'_> {
    const fn new<'a>(variables: &'a HashMap<&str, f64>) -> XprsImpl<'a> {
        XprsImpl { variables }
    }

    fn eval_element(&self, element: &Element) -> Result<f64, EvalError> {
        let res = match *element {
            Element::Number(n) => n,
            Element::Variable(name) => *self.variables.get(name).ok_or_else(
                #[cold]
                || EvalError(name.to_owned()),
            )?,
            Element::UnOp(ref unop) => {
                let operand = self.eval_element(&unop.operand)?;
                #[allow(clippy::unreachable)]
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
                let args = func
                    .args
                    .iter()
                    .map(|arg| self.eval_element(arg))
                    .collect::<Result<Vec<_>, EvalError>>()?;
                func.call(&args)
            },
        };

        Ok(res)
    }

    fn eval_element_unchecked(&self, element: &Element) -> f64 {
        match *element {
            Element::Number(n) => n,
            #[allow(clippy::unwrap_used)]
            Element::Variable(name) => *self.variables.get(name).unwrap(),
            Element::UnOp(ref unop) => {
                let operand = self.eval_element_unchecked(&unop.operand);
                #[allow(clippy::unreachable)]
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
                let left = self.eval_element_unchecked(&binop.lhs);
                let right = self.eval_element_unchecked(&binop.rhs);
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
                let args = func
                    .args
                    .iter()
                    .map(|arg| self.eval_element_unchecked(arg))
                    .collect::<Vec<_>>();
                func.call(&args)
            },
        }
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
    pub fn bind(self, var: &'a str) -> Result<impl Fn(f64) -> f64 + 'a, BindError> {
        if let Some(&needed) = self.vars.iter().next() {
            if var != needed {
                yeet!(BindError::OneVariable(needed.to_owned()));
            }
        }
        Ok(move |val| self.eval_unchecked(&[(var, val)].into()))
    }

    #[inline]
    pub fn bind2(self, var1: &'a str, var2: &'a str) -> Result<impl Fn(f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2| self.eval_unchecked(&[(var1, val1), (var2, val2)].into()))
    }

    #[inline]
    pub fn bind3(self, var1: &'a str, var2: &'a str, var3: &'a str) -> Result<impl Fn(f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3)].into()))
    }

    #[inline]
    pub fn bind4(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str) -> Result<impl Fn(f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4)].into()))
    }

    #[inline]
    pub fn bind5(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str) -> Result<impl Fn(f64, f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4, var5]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4, val5| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5)].into()))
    }

    #[inline]
    pub fn bind6(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4, var5, var6]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4, val5, val6| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6)].into()))
    }

    #[inline]
    pub fn bind7(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4, var5, var6, var7]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4, val5, val6, val7| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7)].into()))
    }

    #[inline]
    pub fn bind8(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str, var8: &'a str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4, var5, var6, var7, var8]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4, val5, val6, val7, val8| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7), (var8, val8)].into()))
    }

    #[inline]
    pub fn bind9(self, var1: &'a str, var2: &'a str, var3: &'a str, var4: &'a str, var5: &'a str, var6: &'a str, var7: &'a str, var8: &'a str, var9: &'a str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64, f64, f64) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from([var1, var2, var3, var4, var5, var6, var7, var8, var9]);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |val1, val2, val3, val4, val5, val6, val7, val8, val9| self.eval_unchecked(&[(var1, val1), (var2, val2), (var3, val3), (var4, val4), (var5, val5), (var6, val6), (var7, val7), (var8, val8), (var9, val9)].into()))
    }

    // NOTE: Too lazy to implement this for more than 9 variables even with Copilot
    // + I don't really think anyone will need more than 9 variables anyway
    #[inline]
    pub fn bind_n<const T: usize>(self, vars: [&'a str; T]) -> Result<impl Fn([f64; T]) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from(vars);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |vals| self.eval_unchecked(&vars.into_iter().zip(vals).collect()))
    }

    #[inline]
    pub fn bind_n_runtime(self, vars: &'a [&'a str]) -> Result<impl Fn(&[f64]) -> Result<f64, EvalError> + 'a, BindError> {
        let variables: HashSet<&str> = vars.iter().copied().collect();
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        // can't drop the closure from returning a result because we can't use the unchecked version
        // because we don't know the length of the slice at compile time
        // it could be different from the length of the slice of variables names
        Ok(move |vals: &[f64]| self.eval(&vars.iter().copied().zip(vals.iter().copied()).collect()))
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum BindError {
    #[error("Variable '{0}' was not provided")]
    OneVariable(String),
    #[error("Variables '{0}' were not provided")]
    MultipleVariables(String),
}

use std::collections::{hash_map::RandomState, hash_set::Difference};
impl BindError {
    fn from_diff(
        missing_vars: Difference<'_, &str, RandomState>,
    ) -> Option<Self> {
        let mut peekable = missing_vars.peekable();
        let mut count: u8 = 0;
        let mut missing_vars_str = String::new();
        while let Some(missing_var) = peekable.next() {
            count += 1;
            missing_vars_str.push_str(missing_var);
            if peekable.peek().is_some() {
                missing_vars_str.push_str(", ");
            }
        }
        match count {
            0 => None,
            1 => Some(Self::OneVariable(missing_vars_str)),
            _ => Some(Self::MultipleVariables(missing_vars_str)),
        }
    }
}
