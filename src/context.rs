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
///     // clone because assert_eq! is used later
///     .with_fn(sin_xprs_func.clone());
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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Context<'names> {
    /// The symbols that are available in the context.
    symbols: HashMap<&'names str, Symbol>,
    /// Optional set of expected variables.
    expected_vars: Option<HashSet<&'names str>>,
}

impl<'names> Context<'names> {
    /// Sets the value of a variable in the context.
    #[inline]
    pub fn set_var(&mut self, name: &'names str, value: impl Into<f64>) {
        self.symbols.insert(name, value.into().into());
    }

    /// Sets the value of a variable in the context, returning the context.
    #[inline]
    #[must_use]
    pub fn with_var(
        mut self,
        name: &'names str,
        value: impl Into<f64>,
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

/// Creates a context from values and inline mathematical functions.
///
/// Fixed-arity functions infer their argument count from the parameter list.
/// A parameter declared as `args: &[f64]` is variadic. Functions capture by move and
/// use shared closure storage, just like [`Function::new_dyn`]. Captures must
/// therefore satisfy its `Send + Sync + 'static` bounds; no `dyn` keyword is needed.
/// Entries are evaluated once, in order, and later entries replace earlier ones.
/// Values accept either identifier keys (`x: 2`) or quoted keys (`"x'": 2`).
/// Use `name(args: xprs::Args<'_>): fallible_expression` for a lazy streaming variadic.
///
/// # Example
///
/// ```
/// use xprs::{context, Parser};
///
/// let offset = 3.0;
/// let context = context! {
///     x: 2,
///     double(x): 2.0 * x,
///     shift(x): x + offset,
///     sum(args: &[f64]): args.iter().sum::<f64>(),
/// };
/// let expression = Parser::new_with_ctx(context)
///     .parse("sum(x, double(2), shift(1))")?;
/// assert_eq!(expression.eval_no_vars()?, 10.0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Functions must use named parameter-list notation:
///
/// ```compile_fail
/// use xprs::context;
/// let context = context! { "double": |x| 2.0 * x };
/// ```
///
/// ```compile_fail
/// use xprs::context;
/// let context = context! { "answer": move || 42.0 };
/// ```
#[macro_export]
macro_rules! context {
    (@key $name:ident) => { stringify!($name) };
    (@key $name:literal) => { $name };
    (@entries $ctx:expr;) => { $ctx };
    (@entries $ctx:expr; $name:ident($arg:ident: &[f64]): $body:expr $(, $($rest:tt)*)?) => {
        $crate::context!(@entries ($ctx).with_fn(
            $crate::xprs_fn!($name($arg: &[f64]): $body)
        ); $($($rest)*)?)
    };
    (@entries $ctx:expr; $name:ident($arg:ident: $args:ty): $body:expr $(, $($rest:tt)*)?) => {
        $crate::context!(@entries ($ctx).with_fn(
            $crate::xprs_fn!($name($arg: $args): $body)
        ); $($($rest)*)?)
    };
    (@entries $ctx:expr; $name:ident($($arg:ident),* $(,)?): $body:expr $(, $($rest:tt)*)?) => {
        $crate::context!(@entries ($ctx).with_fn(
            $crate::xprs_fn!($name($($arg),*): $body)
        ); $($($rest)*)?)
    };
    (@entries $ctx:expr; $name:tt: $(move)? | $($invalid:tt)*) => {
        compile_error!("context! functions must use name(args): expression, not a closure after a key")
    };
    (@entries $ctx:expr; $name:tt: $(move)? || $($invalid:tt)*) => {
        compile_error!("context! functions must use name(args): expression, not a closure after a key")
    };
    (@entries $ctx:expr; $name:tt: $value:expr $(, $($rest:tt)*)?) => {{
        let context = $ctx;
        let value = $value;
        $crate::context!(@entries context.with_var($crate::context!(@key $name), value); $($($rest)*)?)
    }};
    (@entries $ctx:expr; $($invalid:tt)+) => {
        compile_error!("expected name: value, \"name\": value, name(args): expression, or name(args: &[f64]): expression")
    };
    ($($entries:tt)*) => {
        $crate::context!(@entries $crate::Context::default(); $($entries)*)
    };
}
