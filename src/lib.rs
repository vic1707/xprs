/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod element;
#[doc(hidden)]
mod macros;
mod parser;
mod token;
/* Tests */
#[cfg(test)]
mod tests;
/* Exports */
pub use parser::{ErrorKind, ParseError, Parser};

/* Feature safety */
#[cfg(all(feature = "pemdas", feature = "pejmdas"))]
compile_error!(
    "You can't enable both features `pemdas` and `pejmdas` at the same time."
);
#[cfg(not(any(feature = "pemdas", feature = "pejmdas")))]
compile_error!("You must enable either feature `pemdas` or `pejmdas`.");
