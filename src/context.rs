//! This module defines the `Context` struct, which represents the execution context
//! for a mathematical expression evaluator.
//!
//! # Examples
//!
//! ```
//! use xprs::{xprs_fn, Context};
//!
//! let mut context = Context::default();
//!
//! context.set_var("x", 42.0);
//! let sin_func = xprs_fn!("sin", f64::sin, 1);
//! context.set_func(sin_func);
//!
//! let result = context.get_var("x").unwrap();
//! assert_eq!(*result, 42.0);
//!
//! let sin_func_result = context.get_func("sin").unwrap();
//! assert_eq!(sin_func_result.name, "sin");
//! assert_eq!(sin_func_result.nb_args, Some(1));
//! ```

/* Built-in imports */
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::token::Function;

/// Represents the context for the mathematical expression parser.
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
    pub fn with_var<T: Into<f64>>(mut self, name: &'names str, value: T) -> Self {
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
    pub fn with_expected_vars(mut self, expected_vars: HashSet<&'names str>) -> Self {
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
