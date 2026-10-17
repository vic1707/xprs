/* Clippy Config */
#![allow(
    clippy::std_instead_of_core,
    reason = "HashMap and HashSet require std"
)]
/* Built-in imports */
use core::{convert::Infallible, fmt, mem};
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::{
    element::Element, element::Fold as _, token::Operator,
    utils::factorial::factorial,
};

/// Represents a mathematical expression and its variables.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub struct Xprs<'expr> {
    /// The root element of the expression.
    pub root: Element<'expr>,
    /// The set of variables present in the expression.
    pub vars: HashSet<&'expr str>,
}

impl<'input> TryFrom<&'input str> for Xprs<'input> {
    type Error = crate::ParseError;

    #[inline]
    fn try_from(value: &'input str) -> Result<Self, Self::Error> {
        let parser = crate::Parser::default();
        parser.parse(value)
    }
}

impl fmt::Display for Xprs<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl Xprs<'_> {
    /// Evaluates the expression using the provided variable values.
    /// Returns an [`f64`] if the evaluation is successful, or an [`EvalError`] if an error occurs.
    ///
    /// # Errors
    ///
    /// An [`EvalError`] is returned if a variable is not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    /// use std::collections::HashMap;
    ///
    /// let expression = "2 * x + y";
    /// let xprs = Xprs::try_from(expression)?;
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn eval(
        &self,
        variables: &HashMap<&str, f64>,
    ) -> Result<f64, EvalError> {
        XprsImpl::new(|name: &str| variables.get(name).copied())
            .eval_element(&self.root)
    }

    /// Evaluates the expression using the provided variable values without error handling.
    /// Returns an [`f64`] if the evaluation is successful, or panics if an error occurs.
    ///
    /// # Panic
    ///
    /// Use with caution, as it may panic if variable(s) are missing.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    /// # use std::panic;
    /// # macro_rules! assert_panic {
    /// #     ($($t:tt)*) => {
    /// #         panic::catch_unwind(panic::AssertUnwindSafe(|| {
    /// #             $($t)*
    /// #         })).is_err()
    /// #     }
    /// # }
    /// use std::collections::HashMap;
    ///
    /// let expression = "2 * x + y";
    /// let xprs = Xprs::try_from(expression)?;
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn eval_unchecked(&self, variables: &HashMap<&str, f64>) -> f64 {
        XprsImpl::new(|name: &str| variables.get(name).copied())
            .eval_element_unchecked(&self.root)
    }

    /// Simplifies the expression in-place for a single variable.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    ///
    /// let expression = "2 * x + y";
    /// let mut xprs = Xprs::try_from(expression)?;
    ///
    /// assert_eq!(format!("{xprs}"), "((2 * x) + y)");
    /// assert_eq!(xprs.vars, ["x", "y"].into());
    ///
    /// xprs.simplify_for_in_place(("x", 3.0));
    ///
    /// assert_eq!(format!("{xprs}"), "(6 + y)");
    /// assert_eq!(xprs.vars, ["y"].into());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn simplify_for_in_place(&mut self, var: (&str, f64)) {
        let root = mem::replace(&mut self.root, Element::Number(0.0));
        self.root = root.substitute_and_fold(var);
        self.vars.clear();
        self.root.find_variables(&mut self.vars);
    }

    /// Simple wrapper around [`Xprs::eval`] that doesn't require any variables.
    /// This will obviously fail if the expression contains variables.
    ///
    /// # Errors
    ///
    /// An [`EvalError`] is returned if the expression contains variables.
    #[inline]
    pub fn eval_no_vars(&self) -> Result<f64, EvalError> {
        self.eval(&[].into())
    }

    /// Simple wrapper around [`Xprs::eval_unchecked`] that doesn't require any variables.
    /// This will obviously fail if the expression contains variables.
    ///
    /// # Panic
    ///
    /// Use with caution, as it may panic if the expression contains variables.
    #[inline]
    #[must_use]
    pub fn eval_no_vars_unchecked(&self) -> f64 {
        self.eval_unchecked(&[].into())
    }

    /// Simplifies the expression in-place for a single variable and returns the expression.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    ///
    /// let expression = "2 * x + y";
    /// let xprs = Xprs::try_from(expression)?;
    ///
    /// assert_eq!(format!("{xprs}"), "((2 * x) + y)");
    /// assert_eq!(xprs.vars, ["x", "y"].into());
    ///
    /// let simplified_xprs = xprs.simplify_for(("x", 3.0));
    ///
    /// assert_eq!(format!("{simplified_xprs}"), "(6 + y)");
    /// assert_eq!(simplified_xprs.vars, ["y"].into());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn simplify_for(mut self, var: (&str, f64)) -> Self {
        self.simplify_for_in_place(var);
        self
    }

    /// Simplifies the expression in-place for multiple variables.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    ///
    /// let expression = "2 * x + y + 4z";
    /// let mut xprs = Xprs::try_from(expression)?;
    ///
    /// assert_eq!(format!("{xprs}"), "(((2 * x) + y) + (4 * z))");
    /// assert_eq!(xprs.vars, ["x", "y", "z"].into());
    ///
    /// xprs.simplify_for_multiple_in_place(&[("x", 3.0), ("z", 2.0)]);
    ///
    /// assert_eq!(format!("{xprs}"), "((6 + y) + 8)");
    /// assert_eq!(xprs.vars, ["y"].into());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn simplify_for_multiple_in_place(&mut self, vars: &[(&str, f64)]) {
        let mut tmp = mem::replace(&mut self.root, Element::Number(0.0));
        for &var in vars {
            tmp = tmp.substitute_and_fold(var);
        }
        self.root = tmp;
        self.vars.clear();
        self.root.find_variables(&mut self.vars);
    }

    /// Simplifies the expression in-place for multiple variables and returns the expression.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Xprs;
    ///
    /// let expression = "2 * x + y + 4z";
    /// let xprs = Xprs::try_from(expression)?;
    ///
    /// assert_eq!(format!("{xprs}"), "(((2 * x) + y) + (4 * z))");
    /// assert_eq!(xprs.vars, ["x", "y", "z"].into());
    ///
    /// let simplified_xprs = xprs.simplify_for_multiple(&[("x", 3.0), ("z", 2.0)]);
    ///
    /// assert_eq!(format!("{simplified_xprs}"), "((6 + y) + 8)");
    /// assert_eq!(simplified_xprs.vars, ["y"].into());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn simplify_for_multiple(mut self, vars: &[(&str, f64)]) -> Self {
        self.simplify_for_multiple_in_place(vars);
        self
    }

    /// Validates coverage using small slices or the already-prepared large index.
    fn validate_bindings(
        &self,
        names: &[&str],
        indices: Option<&HashMap<&str, usize>>,
    ) -> Result<(), BindError> {
        let missing = self.vars.iter().copied().filter(|name| {
            indices.map_or_else(
                || !names.contains(name),
                |positions| !positions.contains_key(name),
            )
        });
        BindError::from_missing(missing).map_or(Ok(()), Err)
    }

    /// Evaluates bindings using cached indices for large, fully supplied calls.
    fn eval_bound(
        &self,
        names: &[&str],
        values: &[f64],
        indices: Option<&HashMap<&str, usize>>,
    ) -> Result<f64, EvalError> {
        // A short runtime slice must use the last *supplied* duplicate, not the last name.
        let lookup = indices.filter(|_| values.len() >= names.len());
        XprsImpl::new(|name: &str| {
            lookup.map_or_else(
                || {
                    names.iter().zip(values).rev().find_map(
                        |(&bound, &value)| (bound == name).then_some(value),
                    )
                },
                |positions| {
                    positions
                        .get(name)
                        .and_then(|&index| values.get(index))
                        .copied()
                },
            )
        })
        .eval_element(&self.root)
    }

    /// Evaluates fixed bindings whose names and value count were validated.
    #[expect(
        clippy::unwrap_used,
        reason = "fixed binding constructors validate coverage and their argument counts are static"
    )]
    fn eval_bound_unchecked(
        &self,
        names: &[&str],
        values: &[f64],
        indices: Option<&HashMap<&str, usize>>,
    ) -> f64 {
        self.eval_bound(names, values, indices).unwrap()
    }
}

/// An internal struct used for evaluating expressions.
///
/// This struct is responsible for handling the evaluation of individual elements within an expression.
/// It is used by the [`Xprs`] struct to perform evaluations with respect to a given set of variable values.
struct XprsImpl<Lookup> {
    /// Resolves a variable name to its value.
    lookup: Lookup,
}

impl<Lookup> XprsImpl<Lookup>
where
    Lookup: Fn(&str) -> Option<f64>,
{
    /// Creates a new [`XprsImpl`] instance.
    const fn new(lookup: Lookup) -> Self {
        Self { lookup }
    }

    /// Evaluates an element within an expression and returns the result.
    fn eval_element(&self, element: &Element) -> Result<f64, EvalError> {
        let res = match *element {
            Element::Number(n) => n,
            Element::Variable(name) => (self.lookup)(name).ok_or_else(
                #[cold]
                || EvalError(name.to_owned()),
            )?,
            Element::UnOp(ref unop) => {
                let operand = self.eval_element(&unop.operand)?;
                #[expect(
                    clippy::unreachable,
                    reason = "binary operators never appear in a unary expression"
                )]
                match unop.op {
                    Operator::Plus => operand,
                    Operator::Minus => -operand,
                    Operator::Factorial => factorial(operand),
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => unreachable!(),
                }
            },
            Element::BinOp(ref binop) => {
                let left = self.eval_element(&binop.lhs)?;
                let right = self.eval_element(&binop.rhs)?;
                binop.op.eval_binary(left, right)
            },
            Element::Function(ref func) => {
                func.call(func.args.iter().map(|arg| self.eval_element(arg)))?
            },
        };

        Ok(res)
    }

    /// Evaluates an element within an expression without checking for errors.
    fn eval_element_unchecked(&self, element: &Element) -> f64 {
        match *element {
            Element::Number(n) => n,
            #[expect(
                clippy::unwrap_used,
                reason = "unchecked evaluation requires all variables to be supplied"
            )]
            Element::Variable(name) => (self.lookup)(name).unwrap(),
            Element::UnOp(ref unop) => {
                let operand = self.eval_element_unchecked(&unop.operand);
                #[expect(
                    clippy::unreachable,
                    reason = "binary operators never appear in a unary expression"
                )]
                match unop.op {
                    Operator::Plus => operand,
                    Operator::Minus => -operand,
                    Operator::Factorial => factorial(operand),
                    Operator::Times
                    | Operator::Divide
                    | Operator::Power
                    | Operator::Modulo => unreachable!(),
                }
            },
            Element::BinOp(ref binop) => {
                let left = self.eval_element_unchecked(&binop.lhs);
                let right = self.eval_element_unchecked(&binop.rhs);
                binop.op.eval_binary(left, right)
            },
            Element::Function(ref func) => func
                .call::<Infallible>(
                    func.args
                        .iter()
                        .map(|arg| Ok(self.eval_element_unchecked(arg))),
                )
                .unwrap(),
        }
    }
}

/// Represents an error that occurs during expression evaluation, indicating that a variable was not provided.
#[derive(
    Debug, Eq, PartialEq, thiserror::Error, Clone, Hash, Ord, PartialOrd,
)]
#[error("Evaluation error: '{0}' was not provided")]
pub struct EvalError(String);

//////////////////////////////////////////////////////////////////////////////
//  TODO: replace this with variadic generics when it's available & stable  //
//////////////////////////////////////////////////////////////////////////////
#[expect(clippy::too_many_arguments, clippy::type_complexity, reason = "bind methods expose one argument per bound variable")]
#[rustfmt::skip]
impl<'expr> Xprs<'expr> {
    /// Creates a function of one variable based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("x + 2")?;
    /// let func = expression.bind("x");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(3.0);
    /// assert_eq!(result, 5.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind(self, var: &'expr str) -> Result<impl Fn(f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var], None)?;
        Ok(move |val| self.eval_bound_unchecked(&[var], &[val], None))
    }

    /// Creates a function of two [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("x + y")?;
    /// let func = expression.bind2("x", "y");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(3.0, 2.0);
    /// assert_eq!(result, 5.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind2(self, var1: &'expr str, var2: &'expr str) -> Result<impl Fn(f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2], None)?;
        Ok(move |val1, val2| self.eval_bound_unchecked(&[var1, var2], &[val1, val2], None))
    }

    /// Creates a function of three [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("x + y + z")?;
    /// let func = expression.bind3("x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(3.0, 2.0, 1.0);
    /// assert_eq!(result, 6.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind3(self, var1: &'expr str, var2: &'expr str, var3: &'expr str) -> Result<impl Fn(f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3], None)?;
        Ok(move |val1, val2, val3| self.eval_bound_unchecked(&[var1, var2, var3], &[val1, val2, val3], None))
    }

    /// Creates a function of four [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("w + x + y + z")?;
    /// let func = expression.bind4("w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(result, 10.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind4(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str) -> Result<impl Fn(f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4], None)?;
        Ok(move |val1, val2, val3, val4| self.eval_bound_unchecked(&[var1, var2, var3, var4], &[val1, val2, val3, val4], None))
    }

    /// Creates a function of five [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("v + w + x + y + z")?;
    /// let func = expression.bind5("v", "w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0, 5.0);
    /// assert_eq!(result, 15.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind5(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str, var5: &'expr str) -> Result<impl Fn(f64, f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4, var5], None)?;
        Ok(move |val1, val2, val3, val4, val5| self.eval_bound_unchecked(&[var1, var2, var3, var4, var5], &[val1, val2, val3, val4, val5], None))
    }

    /// Creates a function of six [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("u + v + w + x + y + z")?;
    /// let func = expression.bind6("u", "v", "w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    /// assert_eq!(result, 21.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind6(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str, var5: &'expr str, var6: &'expr str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4, var5, var6], None)?;
        Ok(move |val1, val2, val3, val4, val5, val6| self.eval_bound_unchecked(&[var1, var2, var3, var4, var5, var6], &[val1, val2, val3, val4, val5, val6], None))
    }

    /// Creates a function of seven [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("t + u + v + w + x + y + z")?;
    /// let func = expression.bind7("t", "u", "v", "w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
    /// assert_eq!(result, 28.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind7(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str, var5: &'expr str, var6: &'expr str, var7: &'expr str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4, var5, var6, var7], None)?;
        Ok(move |val1, val2, val3, val4, val5, val6, val7| self.eval_bound_unchecked(&[var1, var2, var3, var4, var5, var6, var7], &[val1, val2, val3, val4, val5, val6, val7], None))
    }

    /// Creates a function of eight [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("s + t + u + v + w + x + y + z")?;
    /// let func = expression.bind8("s", "t", "u", "v", "w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
    /// assert_eq!(result, 36.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind8(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str, var5: &'expr str, var6: &'expr str, var7: &'expr str, var8: &'expr str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4, var5, var6, var7, var8], None)?;
        Ok(move |val1, val2, val3, val4, val5, val6, val7, val8| self.eval_bound_unchecked(&[var1, var2, var3, var4, var5, var6, var7, var8], &[val1, val2, val3, val4, val5, val6, val7, val8], None))
    }

    /// Creates a function of nine [`f64`] based on this [`Xprs`] instance.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("r + s + t + u + v + w + x + y + z")?;
    /// let func = expression.bind9("r", "s", "t", "u", "v", "w", "x", "y", "z");
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    /// assert_eq!(result, 45.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind9(self, var1: &'expr str, var2: &'expr str, var3: &'expr str, var4: &'expr str, var5: &'expr str, var6: &'expr str, var7: &'expr str, var8: &'expr str, var9: &'expr str) -> Result<impl Fn(f64, f64, f64, f64, f64, f64, f64, f64, f64) -> f64 + 'expr, BindError> {
        self.validate_bindings(&[var1, var2, var3, var4, var5, var6, var7, var8, var9], None)?;
        Ok(move |val1, val2, val3, val4, val5, val6, val7, val8, val9| self.eval_bound_unchecked(&[var1, var2, var3, var4, var5, var6, var7, var8, var9], &[val1, val2, val3, val4, val5, val6, val7, val8, val9], None))
    }

    // NOTE: Too lazy to implement this for more than 9 variables even with Copilot
    // + I don't really think anyone will need more than 9 variables anyway

    /// Creates a function of any number* of [`f64`] based on this [`Xprs`] instance.
    /// *The number of variables must be known at compile time.
    ///
    /// The returned closure takes an array of [`f64`] as input and returns an [`f64`].
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("a + b + c + d")?;
    /// let func = expression.bind_n(["a", "b", "c", "d"]);
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func([1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(result, 10.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind_n<const T: usize>(self, vars: [&'expr str; T]) -> Result<impl Fn([f64; T]) -> f64 + 'expr, BindError> {
        let indices = (T > 9).then(|| vars.iter().copied().enumerate().map(|(index, name)| (name, index)).collect());
        self.validate_bindings(&vars, indices.as_ref())?;
        Ok(move |values: [f64; T]| self.eval_bound_unchecked(&vars, &values, indices.as_ref()))
    }

    /// Creates a function of any number of [`f64`] based on this [`Xprs`] instance.
    ///
    /// The returned closure takes a slice of [`f64`] as input and 
    /// returns a [`Result`] containing an [`f64`] if the evaluation is successful, or an [`EvalError`] if an error occurs.
    ///
    /// # Errors
    ///
    /// A [`BindError`] is returned if one or more required variables were not provided.
    ///
    /// # Example
    ///
    /// ```
    /// use xprs::Parser;
    ///
    /// let expression = Parser::default().parse("a + b + c + d")?;
    /// let func = expression.bind_n_runtime(&["a", "b", "c", "d"]);
    ///
    /// assert!(func.is_ok());
    /// let func = func?;
    ///
    /// let result = func(&[1.0, 2.0, 3.0, 4.0])?;
    /// assert_eq!(result, 10.0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    pub fn bind_n_runtime(self, vars: &'expr [&'expr str]) -> Result<impl Fn(&[f64]) -> Result<f64, EvalError> + 'expr, BindError> {
        let indices = (vars.len() > 9).then(|| vars.iter().copied().enumerate().map(|(index, name)| (name, index)).collect());
        self.validate_bindings(vars, indices.as_ref())?;
        // can't drop the closure from returning a result because we can't use the unchecked version
        // because we don't know the length of the slice at compile time
        // it could be different from the length of the slice of variables names
        Ok(move |values: &[f64]| self.eval_bound(vars, values, indices.as_ref()))
    }
}

/// Represents errors that occur when binding variables for expression evaluation.
#[derive(
    Debug, Eq, PartialEq, thiserror::Error, Clone, Hash, Ord, PartialOrd,
)]
#[non_exhaustive]
pub enum BindError {
    #[error("Variable '{0}' was not provided")]
    OneVariable(String),
    #[error("Variables '{0}' were not provided")]
    MultipleVariables(String),
}

impl BindError {
    /// Formats missing variable names without allocating on successful bindings.
    fn from_missing<'name>(
        mut missing_vars: impl Iterator<Item = &'name str>,
    ) -> Option<Self> {
        let first = missing_vars.next()?;
        let mut remaining = missing_vars.peekable();
        let multiple = remaining.peek().is_some();
        let mut missing_vars_str = first.to_owned();
        for missing_var in remaining {
            missing_vars_str.push_str(", ");
            missing_vars_str.push_str(missing_var);
        }
        Some(if multiple {
            Self::MultipleVariables(missing_vars_str)
        } else {
            Self::OneVariable(missing_vars_str)
        })
    }
}
