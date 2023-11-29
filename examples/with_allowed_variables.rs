//! Allowed variables example
#![allow(clippy::print_stdout, clippy::use_debug)]
use xprs::{Context, Parser};

fn main() {
    let context = Context::default().with_expected_vars(["x", "y"].into());

    let parser = Parser::new_with_ctx(context);

    let result = parser.parse("x + y"); // OK
    let fail = parser.parse("x + z"); // Error

    println!("{result:#?} {fail:#?}");
}
