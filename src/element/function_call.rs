/* Built-in imports */
use core::fmt;
/* Crate imports */
use super::Element;
use crate::utils::Function;

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

impl fmt::Display for FunctionCall<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: see if there's a better way to do this
        #[allow(clippy::fn_address_comparisons, clippy::unreachable)]
        let func_name = match self.func {
            // sin
            _ if self.func == f64::sin => "sin",
            _ if self.func == f64::sinh => "sinh",
            _ if self.func == f64::asin => "asin",
            _ if self.func == f64::asinh => "asinh",
            // cos
            _ if self.func == f64::cos => "cos",
            _ if self.func == f64::cosh => "cosh",
            _ if self.func == f64::acos => "acos",
            _ if self.func == f64::acosh => "acosh",
            // tan
            _ if self.func == f64::tan => "tan",
            _ if self.func == f64::tanh => "tanh",
            _ if self.func == f64::atan => "atan",
            _ if self.func == f64::atanh => "atanh",
            // log
            _ if self.func == f64::ln => "ln",
            _ if self.func == f64::log10 => "log",
            // roots
            _ if self.func == f64::sqrt => "sqrt",
            _ if self.func == f64::cbrt => "cbrt",
            // misc
            _ if self.func == f64::exp => "exp",
            _ if self.func == f64::abs => "abs",
            _ if self.func == f64::floor => "floor",
            _ if self.func == f64::ceil => "ceil",
            _ if self.func == f64::round => "round",
            _ if self.func == f64::trunc => "trunc",
            // error
            _ => unreachable!(),
        };
        write!(fmt, "{func_name}({})", self.arg)
    }
}
