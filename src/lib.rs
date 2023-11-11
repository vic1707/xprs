/* NIGHTLY Features */
// box-patterns if nightly
#![cfg_attr(NIGHTLY, feature(box_patterns))]
/* Clippy config */
#![deny(warnings)]
#![allow(clippy::pub_use)]
/* Modules */
mod context;
mod element;
mod parser;
mod token;
mod utils;
mod xprs;
/* Tests */
#[cfg(test)]
mod tests;
/* Exports */
pub use context::Context;
pub use parser::{ErrorKind, ParseError, Parser};
pub use token::Function;
pub use xprs::{BindError, EvalError, Xprs};

/* Feature safety */
#[cfg(all(feature = "pemdas", feature = "pejmdas"))]
compile_error!(
    "You can't enable both features `pemdas` and `pejmdas` at the same time."
);
#[cfg(not(any(feature = "pemdas", feature = "pejmdas")))]
compile_error!("You must enable either feature `pemdas` or `pejmdas`.");
