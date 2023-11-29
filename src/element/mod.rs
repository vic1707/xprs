/* Built-in imports */
use core::fmt;
use std::collections::HashSet;
/* Modules */
/// Binary operation module.
mod binop;
/// Function call module.
mod function_call;
/// Simplification's trait module.
mod simplify;
/// Unary operation module.
mod unop;
/* Exports */
pub use binop::BinOp;
pub use function_call::FunctionCall;
pub use simplify::Simplify;
pub use unop::UnOp;

/// Represents an element in the abstract syntax tree (AST).
#[derive(Debug, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Element<'a> {
    /// A number.
    Number(f64),
    /// A binary operation.
    BinOp(Box<BinOp<'a>>),
    /// A unary operation.
    UnOp(Box<UnOp<'a>>),
    /// A function call.
    Function(Box<FunctionCall<'a>>),
    /// A variable.
    Variable(&'a str),
}

impl fmt::Display for Element<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Number(num) => write!(fmt, "{num}"),
            Self::BinOp(ref binop) => write!(fmt, "{binop}"),
            Self::UnOp(ref unop) => write!(fmt, "{unop}"),
            Self::Function(ref func) => write!(fmt, "{func}"),
            Self::Variable(var) => write!(fmt, "{var}"),
        }
    }
}

impl<T> From<T> for Element<'_>
where
    T: Into<f64>,
{
    fn from(num: T) -> Self {
        Self::Number(num.into())
    }
}

impl<'a> From<BinOp<'a>> for Element<'a> {
    fn from(binop: BinOp<'a>) -> Self {
        Self::BinOp(Box::new(binop))
    }
}

impl<'a> From<UnOp<'a>> for Element<'a> {
    fn from(unop: UnOp<'a>) -> Self {
        Self::UnOp(Box::new(unop))
    }
}

impl<'a> From<FunctionCall<'a>> for Element<'a> {
    fn from(func: FunctionCall<'a>) -> Self {
        Self::Function(Box::new(func))
    }
}

impl<'a> Element<'a> {
    /// Finds variables in the element and adds them to the provided set.
    pub(crate) fn find_variables(&self, vars: &mut HashSet<&'a str>) {
        match *self {
            Self::Variable(var) => {
                vars.insert(var);
            },
            Self::BinOp(ref binop) => {
                binop.lhs.find_variables(vars);
                binop.rhs.find_variables(vars);
            },
            Self::UnOp(ref unop) => {
                unop.operand.find_variables(vars);
            },
            Self::Function(ref func) => {
                func.args.iter().for_each(|arg| arg.find_variables(vars));
            },
            Self::Number(_) => (),
        };
    }
}
