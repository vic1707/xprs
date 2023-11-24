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

/// Represents a mathematical expression and its variables.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub struct Xprs<'a> {
    /// The root element of the expression.
    pub root: Element<'a>,
    /// The set of variables present in the expression.
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
    /// Evaluates the expression using the provided variable values.
    ///
    /// # Returns
    ///
    /// A `Result` containing the result of the expression evaluation if successful, or an [`EvalError`] if an error occurs.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::{Xprs, EvalError};
    /// use std::collections::HashMap;
    ///
    /// let expression = "2 * x + y";
    /// let xprs = Xprs::try_from(expression).unwrap();
    ///
    /// let mut variable_values = HashMap::new();
    /// variable_values.insert("x", 3.0);
    /// variable_values.insert("y", 2.0);
    ///
    /// let result = xprs.eval(&variable_values);
    /// assert_eq!(result, Ok(8.0));
    /// 
    /// // we didn't provide the variables, so this should fail
    /// let failed_eval = xprs.eval(&HashMap::new());
    /// assert!(failed_eval.is_err());
    /// ```
    #[inline]
    pub fn eval(
        &self,
        variables: &HashMap<&str, f64>,
    ) -> Result<f64, EvalError> {
        XprsImpl::new(variables).eval_element(&self.root)
    }

    /// Evaluates the expression using the provided variable values without error handling.
    ///
    /// # Returns
    ///
    /// The result of the expression evaluation. Use with caution, as it may panic if variables are not present.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::Xprs;
    /// # macro_rules! assert_panic {
    /// #     ($($t:tt)*) => {
    /// #         std::panic::catch_unwind(|| {
    /// #             $($t)*
    /// #         }).is_err()
    /// #     }
    /// # }
    /// use std::collections::HashMap;
    ///
    /// let expression = "2 * x + y";
    /// let xprs = Xprs::try_from(expression).unwrap();
    ///
    /// let mut variable_values = HashMap::new();
    /// variable_values.insert("x", 3.0);
    /// variable_values.insert("y", 2.0);
    ///
    /// let result = xprs.eval_unchecked(&variable_values);
    /// assert_eq!(result, 8.0);
    /// 
    /// // we didn't provide the variables, so this should panic
    /// assert_panic!(xprs.eval_unchecked(&HashMap::new()));
    #[inline]
    #[must_use]
    pub fn eval_unchecked(&self, variables: &HashMap<&str, f64>) -> f64 {
        XprsImpl::new(variables).eval_element_unchecked(&self.root)
    }

    /// Simplifies the expression in-place for a single variable.
    #[inline]
    pub fn simplify_for_inplace(&mut self, var: (&str, f64)) {
        let mut tmp = trust_me!(ptr::read(&self.root));
        tmp = tmp.simplify_for(var);
        trust_me!(ptr::write(&mut self.root, tmp););
        self.vars.remove(var.0);
    }

    /// Simplifies the expression in-place for a single variable and returns the expression.
    #[inline]
    #[must_use]
    pub fn simplify_for(mut self, var: (&str, f64)) -> Self {
        self.simplify_for_inplace(var);
        self
    }

    /// Simplifies the expression in-place for multiple variables.
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

    /// Simplifies the expression in-place for multiple variables and returns the expression.
    #[inline]
    #[must_use]
    pub fn simplify_for_multiple(mut self, vars: &[(&str, f64)]) -> Self {
        self.simplify_for_multiple_inplace(vars);
        self
    }
}

/// An internal struct used for evaluating expressions.
///
/// This struct is responsible for handling the evaluation of individual elements within an expression.
/// It is used by the [`Xprs`] struct to perform evaluations with respect to a given set of variable values.
struct XprsImpl<'a> {
    /// A reference to the map of variables and their corresponding values.
    variables: &'a HashMap<&'a str, f64>,
}

impl XprsImpl<'_> {
    /// Creates a new [`XprsImpl`] instance.
    const fn new<'a>(variables: &'a HashMap<&str, f64>) -> XprsImpl<'a> {
        XprsImpl { variables }
    }

    /// Evaluates an element within an expression and returns the result.
    ///
    /// # Returns
    ///
    /// The result of the evaluation as a `Result` containing the numeric value or an [`EvalError`] if an error occurs.
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

    /// Evaluates an element within an expression without checking for errors.
    ///
    /// # Returns
    ///
    /// The result of the evaluation as a numeric value. This method assumes that no errors will occur during evaluation.
    /// If an error occurs, the code will panic.
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

/// Represents an error that occurs during expression evaluation, indicating that a variable was not provided.
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[error("Evaluation error: '{0}' was not provided")]
pub struct EvalError(String);

//////////////////////////////////////////////////////////////////////////////
//  TODO: replace this with variadic generics when it's available & stable  //
//////////////////////////////////////////////////////////////////////////////
#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
impl<'a> Xprs<'a> {
    /// Binds a single variable for expression evaluation, returning a function that takes a value for the bound variable.
    ///
    /// # Arguments
    ///
    /// * [`var`] - The variable to bind.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing a closure that takes a single `f64` argument and returns an `f64`. The closure represents
    /// the bound expression. If the variable is not present in the original expression, an error of type `BindError::OneVariable`
    /// is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::{Parser, BindError};
    /// let expression = Parser::default().parse("x + 2").unwrap();
    /// let bound_expression = expression.bind("x");
    ///
    /// match bound_expression {
    ///     Ok(bound_fn) => {
    ///         let result = bound_fn(5.0);
    ///         assert_eq!(result, 7.0); // x + 2, where x is bound to 5.0
    ///     }
    ///     Err(BindError::OneVariable(var)) => {
    ///         println!("Failed to bind: Variable '{}' was not provided.", var);
    ///     }
    ///     _ => {}
    /// }
    /// ```
    #[inline]
    pub fn bind(self, var: &'a str) -> Result<impl Fn(f64) -> f64 + 'a, BindError> {
        if let Some(&needed) = self.vars.iter().next() {
            if var != needed {
                yeet!(BindError::OneVariable(needed.to_owned()));
            }
        }
        Ok(move |val| self.eval_unchecked(&[(var, val)].into()))
    }

    /// Binds two variables for expression evaluation, returning a function that takes two values for the bound variables.
    ///
    /// # Arguments
    ///
    /// * `var1` - The first variable to bind.
    /// * `var2` - The second variable to bind.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing a closure that takes two `f64` arguments and returns an `f64`. The closure represents
    /// the bound expression. If any of the variables are not present in the original expression, an error of type `BindError::MultipleVariables`
    /// is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::{Xprs, BindError};
    /// let expression = Xprs::try_from("x + y").unwrap();
    /// let bound_expression = expression.bind2("x", "y");
    ///
    /// match bound_expression {
    ///     Ok(bound_fn) => {
    ///         let result = bound_fn(3.0, 4.0);
    ///         assert_eq!(result, 7.0); // x + y, where x is bound to 3.0 and y is bound to 4.0
    ///     }
    ///     Err(BindError::MultipleVariables(vars)) => {
    ///         println!("Failed to bind: Variables '{}' were not provided.", vars);
    ///     }
    ///     _ => {}
    /// }
    /// ```
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

        /// Binds a variable for an arbitrary number of arguments, returning a closure that takes an array of values and evaluates the expression.
    ///
    /// # Arguments
    ///
    /// * [`vars`] - An array of variable names to bind.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing a closure that takes an array of `f64` values for the variables and returns the result of the evaluation,
    /// or an error if the variables do not match the expected variables in the expression.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::Xprs;
    ///
    /// let expression = Xprs::try_from("x + y + z").unwrap();
    /// let bound_expression = expression.bind_n(["x", "y", "z"]).unwrap();
    /// let result = bound_expression([1.0, 2.0, 3.0]);
    /// ```
    #[inline]
    pub fn bind_n<const T: usize>(self, vars: [&'a str; T]) -> Result<impl Fn([f64; T]) -> f64 + 'a, BindError> {
        let variables: HashSet<&str> = HashSet::from(vars);
        let missing_vars = self.vars.difference(&variables);
        if let Some(bind_error) = BindError::from_diff(missing_vars) {
            yeet!(bind_error);
        }
        Ok(move |vals| self.eval_unchecked(&vars.into_iter().zip(vals).collect()))
    }

    /// Binds variables for an arbitrary number of arguments at runtime, returning a closure that takes a slice of values and evaluates the expression.
    ///
    /// # Arguments
    ///
    /// * [`vars`] - A slice of variable names to bind.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing a closure that takes a slice of `f64` values for the variables and returns the result of the evaluation,
    /// or an error if the variables do not match the expected variables in the expression.
    ///
    /// # Example
    ///
    /// ```
    /// # use xprs::Xprs;
    ///
    /// let expression = Xprs::try_from("x + y + z").unwrap();
    /// let bound_expression = expression.bind_n_runtime(&["x", "y", "z"]).unwrap();
    /// let result = bound_expression(&[1.0, 2.0, 3.0]);
    /// ```
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

/// Represents errors that occur when binding variables for expression evaluation.
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
    /// Converts a `Difference` iterator of missing variables into a [`BindError`].
    ///
    /// # Returns
    ///
    /// An optional [`BindError`], representing the error if there are missing variables, or [`None`] if there are none.
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
