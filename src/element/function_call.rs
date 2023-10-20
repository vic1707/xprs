/* Built-in imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::misc::Function;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    pub(crate) name: &'a str,
    pub(crate) func: Function,
    pub(crate) arg: Element<'a>,
}

impl<'a> FunctionCall<'a> {
    pub const fn new(name: &'a str, func: Function, arg: Element<'a>) -> Self {
        Self { name, func, arg }
    }

    pub fn new_element(
        name: &'a str,
        func: Function,
        arg: Element<'a>,
    ) -> Element<'a> {
        Element::Function(Box::new(Self::new(name, func, arg)))
    }
}

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}({})", self.name, self.arg)
    }
}
