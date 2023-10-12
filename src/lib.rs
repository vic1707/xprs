/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod context;
mod element;
#[doc(hidden)]
mod macros;
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
pub use xprs::Xprs;

/* Feature safety */
#[cfg(all(feature = "pemdas", feature = "pejmdas"))]
compile_error!(
    "You can't enable both features `pemdas` and `pejmdas` at the same time."
);
#[cfg(not(any(feature = "pemdas", feature = "pejmdas")))]
compile_error!("You must enable either feature `pemdas` or `pejmdas`.");
