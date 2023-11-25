/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Function};

/// Represents a function call in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    pub(crate) desc: Function,
    pub(crate) args: Vec<Element<'a>>,
}

impl<'a> FunctionCall<'a> {
    /// Creates a new [`FunctionCall`] from the function call components.
    const fn new(desc: Function, args: Vec<Element<'a>>) -> Self {
        Self { desc, args }
    }

    /// Creates a new `Element::Function` from the function call components.
    pub(crate) fn new_element(
        desc: Function,
        args: Vec<Element<'a>>,
    ) -> Element<'a> {
        Element::Function(Box::new(Self::new(desc, args)))
    }

    /// Calls the function with the provided arguments.
    pub(crate) fn call(&self, args: &[f64]) -> f64 {
        (self.desc.func)(args)
    }
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>();
        write!(fmt, "{}({})", self.desc.name, args.join(", "))
    }
}
