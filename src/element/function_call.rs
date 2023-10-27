/* Built-in imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::token::Function;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    pub(crate) func: &'a Function<'a>,
    pub(crate) args: Vec<Element<'a>>,
}

impl<'a> FunctionCall<'a> {
    pub const fn new(func: &'a Function<'a>, args: Vec<Element<'a>>) -> Self {
        Self { func, args }
    }

    pub fn new_element(
        func: &'a Function<'a>,
        args: Vec<Element<'a>>,
    ) -> Element<'a> {
        Element::Function(Box::new(Self::new(func, args)))
    }

    pub fn call(&self, args: &[f64]) -> f64 {
        (self.func.func)(args)
    }
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self
            .args
            .iter()
            .map(|arg| format!("{arg}"))
            .collect::<Vec<_>>();
        write!(fmt, "{}({})", self.func.name, args.join(", "))
    }
}
