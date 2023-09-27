/* Built-in imports */
#![allow(clippy::std_instead_of_core)]
use std::error::Error;
use std::io::{self, Write};
/* Crate imports */
use xprs::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = Parser::default();
    let mut input = String::new();

    println!("Welcome to the XPRS REPL!");
    println!("Feel free to type in expressions and see the AST.");

    loop {
        print!(">> ");
        io::stdout().lock().flush()?;
        io::stdin().read_line(&mut input)?;
        match &input {
            line if line.trim() == "exit" => break,
            line => match parser.parse(line) {
                Ok(ast) => println!("{ast:#?}",),
                Err(err) => println!("{:?}", miette::Report::new(err)),
            },
        }
        input.clear();
    }

    println!("Bye!");
    Ok(())
}
