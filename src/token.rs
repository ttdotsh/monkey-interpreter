#[derive(Debug, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

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
    If,
    Else,
    Return,
    True,
    False,

    Illegal,
    Eof,
}
