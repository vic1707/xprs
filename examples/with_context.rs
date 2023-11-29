//! Context example
#![allow(clippy::print_stdout, clippy::default_numeric_fallback)]
use std::error::Error;
use xprs::{xprs_fn, Context, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = Context::default()
        .with_fn(xprs_fn!("double", |x| 2. * x, 1))
        .with_var("foo", 1.0);
    context.set_var("bar", 2.0);

    let xprs = Parser::new_with_ctx(context).parse("double(foo) + bar")?;
    println!("double(foo) + bar = {}", xprs.eval_no_vars()?);

    Ok(())
}
