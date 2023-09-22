/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod element;
mod parser;
mod token;
mod trust_me;
mod yeet;
/* Tests */
#[cfg(test)]
mod tests;
/* Exports */
pub use parser::Parser;
