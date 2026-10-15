//! Mathematical expression parsing and evaluation.
#![cfg_attr(doc, doc = include_str!("../README.md"))]
/* Nightly features */
#![cfg_attr(nightly, feature(float_gamma))]
/* Clippy config */
#![allow(
    clippy::pub_use,
    clippy::needless_doctest_main,
    reason = "re-export the public API and keep examples runnable as programs"
)]
/* Modules */
/// The context of the parser.
mod context;
/// The element of the abstract syntax tree (AST).
mod element;
/// The parser module.
mod parser;
/// The token module.
mod token;
/// The utilities module.
mod utils;
/// The expression module.
mod xprs;
/* Tests */
#[cfg(test)]
mod tests;
/* Exports */
pub use crate::context::{Context, Symbol};
pub use crate::parser::{ErrorKind, ParseError, Parser};
pub use crate::token::Function;
pub use crate::xprs::{BindError, EvalError, Xprs};

/* Feature safety */
#[cfg(all(feature = "pemdas", feature = "pejmdas"))]
compile_error!(
    "You can't enable both features `pemdas` and `pejmdas` at the same time."
);
#[cfg(not(any(feature = "pemdas", feature = "pejmdas")))]
compile_error!("You must enable either feature `pemdas` or `pejmdas`.");
