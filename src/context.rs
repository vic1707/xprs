/* Built-in imports */
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::token::Function;

/// Represents the context for the mathematical expression parser.
///
/// # Examples
///
/// ```
/// use xprs::{Context, xprs_fn};
///
/// let sin_xprs_func = xprs_fn!("sin", f64::sin, 1);
/// let mut context = Context::default()
///     .with_expected_vars(["y"].into())
///     .with_var("x", 42.0)
///     .with_func(sin_xprs_func);
///
/// let x_var = context.get_var("x");
/// assert_eq!(x_var, Some(&42.0));
///
/// let sin_func = context.get_func("sin");
/// assert_eq!(sin_func, Some(&sin_xprs_func));
///
/// let expected_vars = context.get_expected_vars();
/// assert_eq!(expected_vars, Some(&["y"].into()));
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct Context<'names> {
    /// Variables defined in the context.
    vars: HashMap<&'names str, f64>,
    /// Functions defined in the context.
    funcs: HashMap<&'names str, Function>,
    /// Optional set of expected variables.
    expected_vars: Option<HashSet<&'names str>>,
}

impl<'names> Context<'names> {
    /// Sets the value of a variable in the context.
    #[inline]
    pub fn set_var<T: Into<f64>>(&mut self, name: &'names str, value: T) {
        self.vars.insert(name, value.into());
    }

    /// Sets the value of a variable in the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_var<T: Into<f64>>(
        mut self,
        name: &'names str,
        value: T,
    ) -> Self {
        self.vars.insert(name, value.into());
        self
    }

    /// Sets a function in the context.
    #[inline]
    pub fn set_func(&mut self, func: Function) {
        self.funcs.insert(func.name, func);
    }

    /// Sets a function in the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_func(mut self, func: Function) -> Self {
        self.funcs.insert(func.name, func);
        self
    }

    /// Sets the expected variables for the context.
    #[inline]
    pub fn set_expected_vars(&mut self, expected_vars: HashSet<&'names str>) {
        self.expected_vars = Some(expected_vars);
    }

    /// Sets the expected variables for the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_expected_vars(
        mut self,
        expected_vars: HashSet<&'names str>,
    ) -> Self {
        self.expected_vars = Some(expected_vars);
        self
    }

    /// Retrieves the value of a variable from the context.
    #[inline]
    #[must_use]
    pub fn get_var(&self, name: &str) -> Option<&f64> {
        self.vars.get(name)
    }

    /// Retrieves a function from the context.
    #[inline]
    #[must_use]
    pub fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    /// Retrieves the set of expected variables from the context.
    #[inline]
    #[must_use]
    pub const fn get_expected_vars(&self) -> Option<&HashSet<&str>> {
        self.expected_vars.as_ref()
    }
}
