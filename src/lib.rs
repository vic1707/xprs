//! # Xprs
//!
//! **Xprs** is a flexible and extensible mathematical expression parser and evaluator for Rust, designed for simplicity and ease of use.
//!
//! ## Features
//!
//! - **Expression Parsing:** Parse mathematical expressions into a structured representation for easy evaluation.
//! - **Expression Evaluation:** Evaluate expressions with variable bindings, supporting basic arithmetic operations and functions.
//! - **Error Handling:** Provides detailed error information for parsing and evaluation failures.
//!
//! ## Quick Start
//!
//! Add the following to your `Cargo.toml` to use the library:
//!
//! ```toml
//! [dependencies]
//! xprs = "0.1"
//! ```
//!
//! Here's a simple example of using **Xprs**:
//!
//! ```rust
//! use xprs::{Parser, BindError};
//!
//!     // Parse a mathematical expression
//!     let expression = Parser::default().parse("2 * x + 3").unwrap();
//!     // or `let expression = Xprs::try_from("2 * x + 3")?``
//!
//!     // Bind a variable and evaluate the expression
//!     match expression.bind("x") {
//!         Ok(bound_fn) => {
//!             let result = bound_fn(5.0);
//!             println!("Result: {}", result); // Output: 13.0
//!         }
//!         Err(BindError::OneVariable(var)) => {
//!             println!("Failed to bind: Variable '{}' was not provided.", var);
//!         }
//!         _ => {}
//!     }
//! ```
//!
//! ## Features
//!
//! - **`pemdas` (default):** Enables support for PEMDAS/BODMAS order of operations (Parentheses/Exponents/Multiplication and Division/Addition and Subtraction).
//! - **`pejmdas`:** Enables support for PEJMDAS order of operations (Parentheses/Exponents/Modulo/Division and Multiplication/Addition and Subtraction).
//!
//! ## Nightly Features
//!
//! - **`box_patterns` (nightly):** Enables the use of box patterns if nightly feature is enabled.
//!
//! ## Clippy Configuration
//!
//! The library has configured Clippy to deny warnings and allow `pub use` statements.
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE](https://github.com/your_username/your_project/blob/master/LICENSE) file for details.
//!
//! ## Contributing
//!
//! Contributions are welcome! Feel free to open issues or submit pull requests on [GitHub](https://github.com/your_username/your_project).

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
