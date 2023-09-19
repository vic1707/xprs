/* Crate imports */
use super::Element;
use crate::token::Function;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    func: Function,
    arg: Element<'a>,
}

impl<'a> FunctionCall<'a> {
    pub const fn new(func: Function, arg: Element<'a>) -> Self {
        Self { func, arg }
    }
}
