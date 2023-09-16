/* Built-in imports */
#![allow(clippy::std_instead_of_core)]
use std::error::Error;
use std::io::{self, Write};
/* Crate imports */
use xprs::{Lexer, Token};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    println!("Welcome to the XPRS REPL!");
    println!("Feel free to type in expressions and see the Tokens & AST.");

    loop {
        print!(">> ");
        io::stdout().lock().flush()?;
        io::stdin().read_line(&mut input)?;
        match &input {
            line if line.trim() == "exit" => break,
            line => {
                let tokens = Lexer::new(line)
                    .collect::<Result<Vec<Token<'_>>, &str>>()?;
                println!("{tokens:#?}");
            },
        }
        input.clear();
    }

    println!("Bye!");
    Ok(())
}
