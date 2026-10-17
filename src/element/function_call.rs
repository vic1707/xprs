/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{EvalError, element::Element, token::Function};

/// Represents a function call in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FunctionCall<'expr> {
    /// The function being called.
    pub(crate) desc: Function,
    /// The arguments of the function call.
    pub(crate) args: Box<[Element<'expr>]>,
}

impl<'expr> FunctionCall<'expr> {
    /// Creates a new [`FunctionCall`] from the function call components.
    fn new(desc: Function, args: Vec<Element<'expr>>) -> Self {
        Self {
            desc,
            args: args.into_boxed_slice(),
        }
    }

    /// Creates a new `Element::Function` from the function call components.
    pub(crate) fn new_element(
        desc: Function,
        args: Vec<Element<'expr>>,
    ) -> Element<'expr> {
        Element::Function(Box::new(Self::new(desc, args)))
    }

    /// Calls the function from fallible argument values, preserving their order.
    pub(crate) fn call(
        &self,
        args: impl ExactSizeIterator<Item = Result<f64, EvalError>>,
    ) -> Result<f64, EvalError> {
        self.desc.call(args)
    }
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.desc.name)?;
        let mut separator = "";
        for arg in &self.args {
            write!(f, "{separator}{arg}")?;
            separator = ", ";
        }
        f.write_str(")")
    }
}
