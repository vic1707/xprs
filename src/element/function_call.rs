/* Built-in imports */
use core::fmt;
/* Crate imports */
use crate::{element::Element, token::Function};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    pub(crate) desc: Function<'a>,
    pub(crate) args: Vec<Element<'a>>,
}

impl<'a> FunctionCall<'a> {
    const fn new(desc: Function<'a>, args: Vec<Element<'a>>) -> Self {
        Self { desc, args }
    }

    pub fn new_element(
        desc: Function<'a>,
        args: Vec<Element<'a>>,
    ) -> Element<'a> {
        Element::Function(Box::new(Self::new(desc, args)))
    }

    pub fn call(&self, args: &[f64]) -> f64 {
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
