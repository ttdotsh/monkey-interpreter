#[derive(Debug, PartialEq, Clone)]
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

impl Token {
    pub fn extract_literal(&mut self) -> Option<String> {
        return match self {
            Token::Ident(s) => Some(std::mem::take(s)),
            Token::Int(s) => Some(std::mem::take(s)),
            _ => None,
        };
    }

    pub fn is(&self, token: &Self) -> bool {
        if self == token {
            return true;
        }

        return match (self, token) {
            (Token::Ident(_), Token::Ident(_)) => true,
            (Token::Int(_), Token::Int(_)) => true,
            _ => false,
        };
    }
}
