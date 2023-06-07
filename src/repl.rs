use std::io::{BufRead, Result, Write};

use crate::lexer::Lexer;

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    writeln!(writer, "Give the monkey some commands!")?;

    loop {
        write!(writer, "{} -> ", "🐒")?;
        writer.flush()?;

        let mut buffer = String::new();
        _ = reader.read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer);

        lexer
            .into_iter()
            .filter_map(|t| t.ok())
            .try_for_each(|t| writeln!(&mut writer, "{:?}", t))?;
        writer.flush()?;
    }
}
