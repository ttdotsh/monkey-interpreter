#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(i32),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,

    // Keywords
    Let,
    Function,
}
