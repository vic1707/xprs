/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Function};

/// Represents a function call in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FunctionCall<'expr> {
    /// The function being called.
    pub(crate) desc: Function,
    /// The arguments of the function call.
    pub(crate) args: Vec<Element<'expr>>,
}

impl<'expr> FunctionCall<'expr> {
    /// Creates a new [`FunctionCall`] from the function call components.
    const fn new(desc: Function, args: Vec<Element<'expr>>) -> Self {
        Self { desc, args }
    }

    /// Creates a new `Element::Function` from the function call components.
    pub(crate) fn new_element(
        desc: Function,
        args: Vec<Element<'expr>>,
    ) -> Element<'expr> {
        Element::Function(Box::new(Self::new(desc, args)))
    }

    /// Calls the function with the provided arguments.
    pub(crate) fn call(&self, args: &[f64]) -> f64 {
        (self.desc.func)(args)
    }
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>();
        write!(f, "{}({})", self.desc.name, args.join(", "))
    }
}
