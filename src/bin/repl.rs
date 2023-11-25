/* Clippy config */
//! XPRS REPL
#![allow(
    clippy::expect_used,
    clippy::print_stdout,
    clippy::unreachable,
    clippy::use_debug
)]
/* Built-in imports */
#![allow(clippy::std_instead_of_core)]
use std::collections::{HashMap, HashSet};
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
                Ok(ast) => {
                    println!("Interpreted as: {ast}");
                    println!("{ast:#?}");
                    println!("Variables: {:?}", ast.vars);
                    let variables = ask_for_variables(&ast.vars);
                    println!(
                        "Evaluated: {:#?}",
                        ast.eval(&variables).unwrap_or_else(|_| unreachable!())
                    );
                },
                Err(err) => println!("{:?}", miette::Report::new(err)),
            },
        }
        input.clear();
    }

    println!("Bye!");
    Ok(())
}

/// Ask the user for the value of the variables in the given set.
fn ask_for_variables<'a>(set: &'a HashSet<&str>) -> HashMap<&'a str, f64> {
    if !set.is_empty() {
        println!("Please enter the following variables:");
    }

    set.iter().fold(HashMap::new(), |mut acc, var| {
        print!("{var}: ");
        io::stdout().lock().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        acc.insert(
            var,
            input
                .trim()
                .parse()
                .expect("Failed to parse input as a number"),
        );
        acc
    })
}
