/* Built-in imports */
#![allow(clippy::std_instead_of_core)]
use std::error::Error;
/* Crate imports */
use xprs::Parser;

#[allow(clippy::print_stdout, clippy::use_debug)]
fn main() -> Result<(), Box<dyn Error>> {
    let parser = Parser::default();
    let mut xprs = parser.parse("x + (2 - 5y)x")?;
    xprs.simplify_for_inplace3(("x", 2.0_f64));
    xprs.simplify_for_inplace3(("y", 2.0_f64));
    println!("{xprs}");
    Ok(())
}
