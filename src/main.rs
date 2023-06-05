use std::io::{stdin, stdout, Result};

mod lexer;
mod repl;
mod token;

fn main() -> Result<()> {
    repl::start(stdin().lock(), stdout())?;
    Ok(())
}
