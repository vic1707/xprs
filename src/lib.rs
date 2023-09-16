/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod lexer;
mod token;
mod trust_me;
mod yeet;
/* Exports */
pub use lexer::Lexer;
pub use token::Token;
