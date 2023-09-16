/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod constants;
mod functions;
mod operators;
/* Exports */
pub use constants::Constant;
pub use functions::Function;
pub use operators::Operator;

#[derive(Debug, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Token<'a> {
    Number(f64),
    Constant(Constant),
    Function(Function),
    Operator(Operator),
    Variable(&'a str),
    /// Left parenthesis.
    LParen,
    /// Right parenthesis.
    RParen,
}
