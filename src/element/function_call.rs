/* Crate imports */
use super::Element;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct FunctionCall<'a> {
    func: fn(f64) -> f64,
    arg: Element<'a>,
}

impl<'a> FunctionCall<'a> {
    pub const PRECEDENCE: usize = 5;

    pub const fn new(func: fn(f64) -> f64, arg: Element<'a>) -> Self {
        Self { func, arg }
    }
}
