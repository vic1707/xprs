/* Built-in imports */
use std::collections::{HashMap, HashSet};
/* Crate imports */
use crate::token::Function;

/// Represents a symbol in the context.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[non_exhaustive]
pub enum Symbol {
    /// A variable.
    Variable(f64),
    /// A function.
    Function(Function),
}

impl From<f64> for Symbol {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Variable(value)
    }
}

impl From<Function> for Symbol {
    #[inline]
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

/// Represents the context for the mathematical expression parser.
///
/// # Examples
///
/// ```
/// use xprs::{Context, Symbol, xprs_fn};
///
/// let sin_xprs_func = xprs_fn!("sin", f64::sin, 1);
/// let mut context = Context::default()
///     .with_expected_vars(["y"].into())
///     .with_var("x", 42.0)
///     .with_fn(sin_xprs_func);
///
/// let x_var = context.get("x");
/// assert_eq!(x_var, Some(&Symbol::Variable(42.0)));
///
/// let sin_func = context.get("sin");
/// assert_eq!(sin_func, Some(&Symbol::Function(sin_xprs_func)));
///
/// let expected_vars = context.get_expected_vars();
/// assert_eq!(expected_vars, Some(&["y"].into()));
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct Context<'names> {
    /// The symbols that are available in the context.
    symbols: HashMap<&'names str, Symbol>,
    /// Optional set of expected variables.
    expected_vars: Option<HashSet<&'names str>>,
}

impl<'names> Context<'names> {
    /// Sets the value of a variable in the context.
    #[inline]
    pub fn set_var<T: Into<f64>>(&mut self, name: &'names str, value: T) {
        self.symbols.insert(name, value.into().into());
    }

    /// Sets the value of a variable in the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_var<T: Into<f64>>(
        mut self,
        name: &'names str,
        value: T,
    ) -> Self {
        self.symbols.insert(name, value.into().into());
        self
    }

    /// Sets a function in the context.
    #[inline]
    pub fn set_fn(&mut self, func: Function) {
        self.symbols.insert(func.name, func.into());
    }

    /// Sets a function in the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_fn(mut self, func: Function) -> Self {
        self.symbols.insert(func.name, func.into());
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

    /// Sets the symbols for the context.
    #[inline]
    #[must_use]
    pub fn with_symbols(
        mut self,
        symbols: HashMap<&'names str, Symbol>,
    ) -> Self {
        self.symbols = symbols;
        self
    }

    /// Returns the value of a symbol in the context.
    #[inline]
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Retrieves the set of expected variables from the context.
    #[inline]
    #[must_use]
    pub const fn get_expected_vars(&self) -> Option<&HashSet<&str>> {
        self.expected_vars.as_ref()
    }
}
