/* Modules */
mod binop;
mod function_call;
mod unop;
/* Exports */
pub use binop::BinOp;
pub use function_call::FunctionCall;
pub use unop::UnOp;

#[derive(Debug, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Element<'a> {
    Number(f64),
    BinOp(Box<BinOp<'a>>),
    UnOp(Box<UnOp<'a>>),
    Function(Box<FunctionCall<'a>>),
    Variable(&'a str),
}
