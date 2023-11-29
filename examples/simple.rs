//! Simple example
#![allow(clippy::print_stdout)]
use std::error::Error;
use xprs::Xprs;

fn main() -> Result<(), Box<dyn Error>> {
    let xprs = Xprs::try_from("1 + sin(2) * 3")?;
    println!("1 + sin(2) * 3 = {}", xprs.eval_no_vars()?);

    Ok(())
}
