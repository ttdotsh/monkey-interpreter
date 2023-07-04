#[derive(Debug, Default, PartialEq, Clone)]
pub enum Token {
    /* Identifiers and Literals */
    Ident(String),
    Int(String),

    /* Operators */
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

    /* Delimiters */
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,

    /* Keywords */
    Let,
    Function,
    If,
    Else,
    Return,
    True,
    False,

    /* Endings */
    #[default]
    Eof,
    Illegal,
}

impl Token {
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

    pub fn is_ident(&self) -> bool {
        return matches!(self, Token::Ident(_));
    }
}
