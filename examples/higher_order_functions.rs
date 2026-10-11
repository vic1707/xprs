//! Higher order functions example.
#![allow(clippy::print_stdout, reason = "examples print their results")]
use core::error::Error;
use xprs::{Context, Parser, Xprs, xprs_fn};

fn main() -> Result<(), Box<dyn Error>> {
    let xprs_hof = Xprs::try_from("2x + y")?;
    let fn_hof = xprs_hof.bind2("x", "y")?;
    let hof = xprs_fn!("hof", dyn fn_hof, 2);
    let ctx = Context::default().with_fn(hof);
    let parser = Parser::new_with_ctx(ctx);

    let xprs = parser.parse("hof(2, 3)")?;

    println!("hof(2, 3) = {}", xprs.eval_no_vars()?);

    Ok(())
}
