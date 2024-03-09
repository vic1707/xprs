//! Simple example
#![allow(
    clippy::print_stdout,
    clippy::default_numeric_fallback,
    clippy::shadow_reuse
)]
use std::error::Error;
use xprs::Xprs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut xprs = Xprs::try_from("w + sin(x + 2y) * (3 * z)")?;

    println!("{xprs}"); // (w + (sin((x + (2 * y))) * (3 * z)))

    xprs.simplify_for_in_place(("z", 4.0));

    println!("{xprs}"); // (w + (sin((x + (2 * y))) * 12))

    let xprs = xprs.simplify_for_multiple(&[("x", 1.0), ("y", 2.0)]);

    println!("{xprs}"); // (w + -11.507091295957661)

    Ok(())
}
