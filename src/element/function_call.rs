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
    pub(crate) fn call<Error>(
        &self,
        args: impl ExactSizeIterator<Item = Result<f64, Error>>,
    ) -> Result<f64, Error> {
        // ponytail: stack space covers fixed arities 0..=9; larger variadics use Vec.
        let mut small = [0.0_f64; 9];
        if let Some(values) = small.get_mut(..args.len()) {
            for (value, arg) in values.iter_mut().zip(args) {
                *value = arg?;
            }
            Ok((self.desc.func)(values))
        } else {
            let mut values = Vec::with_capacity(args.len());
            for arg in args {
                values.push(arg?);
            }
            Ok((self.desc.func)(&values))
        }
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
