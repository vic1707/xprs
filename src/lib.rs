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
pub use parser::{Error, ErrorKind, Parser};
