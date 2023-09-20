/* Clippy config */
#![allow(clippy::pub_use)]
/* Modules */
mod element;
mod lexer;
mod parser;
mod token;
mod trust_me;
mod yeet;
/* Exports */
pub use element::Element;
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::Token;
