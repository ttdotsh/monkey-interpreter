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

const HELP: &str = r#"
help:      prints this message
clear:     clears the screen
exit:      exits the repl
monkey:    prints the monkey
<source>:  parsed and printed AST
"#;

fn main() -> Result<()> {
    let reader = stdin().lock();
    let writer = stdout().lock();
    repl(reader, writer)?;
    Ok(())
}

fn repl<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> Result<()> {
    write!(
        writer,
        "{}This is the Monkey programming language!\nOptions: <help> | <clear> | <exit>\n\n",
        MONKEY_FACE
    )?;

    loop {
        write!(writer, "{} -> ", "🐒")?;
        writer.flush()?;

        let mut line = String::new();
        reader.read_line(&mut line)?;
        line = line
            .chars()
            .filter(|ch| *ch != '\n' && *ch != '\r')
            .collect();

        match line.as_str() {
            "help" => writeln!(writer, "{}", HELP)?,
            "clear" => write!(writer, "{escape}c", escape = '\x1b' as char)?,
            "monkey" => writeln!(writer, "{}", MONKEY_FACE)?,
            "exit" => return Ok(()),
            src => {
                let lexer = Lexer::new(src.into());
                let mut parser = Parser::new(lexer);
                let program = parser.parse();

                if parser.errors.is_empty() {
                    program
                        .iter()
                        .try_for_each(|s| writeln!(&mut writer, "{}", s))?;
                } else {
                    writeln!(writer, "Woah, we ran into some errors here:")?;
                    parser
                        .errors
                        .into_iter()
                        .try_for_each(|e| writeln!(writer, "\t{:?}", e))?;
                    writeln!(writer, "Stop monkeying around!")?;
                }
            }
        }

        writer.flush()?;
    }
}
