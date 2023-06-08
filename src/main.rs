use std::io::{stdin, stdout, Result};

mod ast;
mod lexer;
mod repl;
mod token;

fn main() -> Result<()> {
    let reader = stdin().lock();
    let writer = stdout().lock();
    repl::start(reader, writer)?;
    Ok(())
}
