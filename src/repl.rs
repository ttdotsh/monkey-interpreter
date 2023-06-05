use std::io::{BufRead, Result, Write};

use crate::lexer::Lexer;
use crate::token::Token;

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    println!("Starting 🐒 REPL");
    loop {
        let mut buffer = String::new();
        _ = reader.read_line(&mut buffer)?;
        let mut lexer = Lexer::new(buffer);
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Eof => break,
                _ => writeln!(writer, "{:?}", token)?,
            }
        }
        writer.flush()?;
    }
}
