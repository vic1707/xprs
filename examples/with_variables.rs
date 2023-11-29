//! Variables example
#![allow(clippy::print_stdout, clippy::default_numeric_fallback)]
use std::error::Error;
use xprs::Xprs;

fn main() -> Result<(), Box<dyn Error>> {
    let xprs = Xprs::try_from("1 + sin(2) * x")?;
    println!("1 + sin(2) * x = {}", xprs.eval(&[("x", 3.0)].into())?);

    Ok(())
}
