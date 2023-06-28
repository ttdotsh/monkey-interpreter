use monkey_interpreter::{lex::Lexer, parse::Parser};
use std::io::{stdin, stdout, BufRead, Result, Write};

const MONKEY_FACE: &str = r#"
           __,__
  .--.  .-"     "-.  .--.
 / .. \/  .-. .-.  \/ .. \
| |  '|  /   Y   \  |'  | |
| \   \  \ 0 | 0 /  /   / |
 \ '- ,\.-"""""""-./, -' /
  ''-' /_   ^ ^   _\ '-''
      |  \._   _./  |
       \  \ '~' /  /
        '._'-=-'_.'
          '-----'
"#;

fn main() -> Result<()> {
    let reader = stdin().lock();
    let writer = stdout().lock();
    repl(reader, writer)?;
    Ok(())
}

fn repl<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    // writeln!(writer, "Give the monkey some commands!")?;
    write!(writer, "{}\nGive the monkey some commands!\n", MONKEY_FACE)?;

    loop {
        write!(writer, "{} -> ", "🐒")?;
        writer.flush()?;

        let mut buffer = String::new();
        _ = reader.read_line(&mut buffer)?;
        let lexer = Lexer::new(buffer);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if parser.errors.is_empty() {
            program
                .statements
                .into_iter()
                .try_for_each(|s| writeln!(&mut writer, "{}", s))?;
        } else {
            writeln!(writer, "Woah, we ran into some errors here:")?;
            parser
                .errors
                .into_iter()
                .try_for_each(|e| writeln!(writer, "\t{:?}", e))?;
            writeln!(writer, "Stop monkeying around!")?;
        }

        writer.flush()?;
    }
}
