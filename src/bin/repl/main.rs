use monkey_interpreter::lex::Lexer;
use std::io::{stdin, stdout, BufRead, Result, Write};

fn main() -> Result<()> {
    let reader = stdin().lock();
    let writer = stdout().lock();
    repl(reader, writer)?;
    Ok(())
}

pub fn repl<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    writeln!(writer, "Give the monkey some commands!")?;

    loop {
        write!(writer, "{} -> ", "🐒")?;
        writer.flush()?;

        let mut buffer = String::new();
        _ = reader.read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer);

        lexer
            .into_iter()
            .try_for_each(|t| writeln!(&mut writer, "{:?}", t))?;
        writer.flush()?;
    }
}
