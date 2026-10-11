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
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[non_exhaustive]
pub enum Element<'expr> {
    /// A number.
    Number(f64),
    /// A binary operation.
    BinOp(Box<BinOp<'expr>>),
    /// A unary operation.
    UnOp(Box<UnOp<'expr>>),
    /// A function call.
    Function(Box<FunctionCall<'expr>>),
    /// A variable.
    Variable(&'expr str),
}

impl fmt::Display for Element<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Number(num) => write!(f, "{num}"),
            Self::BinOp(ref binop) => write!(f, "{binop}"),
            Self::UnOp(ref unop) => write!(f, "{unop}"),
            Self::Function(ref func) => write!(f, "{func}"),
            Self::Variable(var) => write!(f, "{var}"),
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

impl<'expr> From<BinOp<'expr>> for Element<'expr> {
    fn from(binop: BinOp<'expr>) -> Self {
        Self::BinOp(Box::new(binop))
    }
}

impl<'expr> From<UnOp<'expr>> for Element<'expr> {
    fn from(unop: UnOp<'expr>) -> Self {
        Self::UnOp(Box::new(unop))
    }
}

impl<'expr> From<FunctionCall<'expr>> for Element<'expr> {
    fn from(func: FunctionCall<'expr>) -> Self {
        Self::Function(Box::new(func))
    }
}

impl<'expr> Element<'expr> {
    /// Finds variables in the element and adds them to the provided set.
    pub(crate) fn find_variables(&self, vars: &mut HashSet<&'expr str>) {
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
        }
    }
}
