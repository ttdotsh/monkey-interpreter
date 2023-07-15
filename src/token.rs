#[derive(Debug, Default, PartialEq, Clone)]
pub enum Token<'a> {
    /* Identifiers and Literals */
    // Ident(String),
    // Int(String),
    Ident(&'a str),
    Int(&'a str),

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

impl Token<'_> {
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

    pub fn literal(&self) -> &str {
        match *self {
            Token::Ident(s) | Token::Int(s) => s,
            _ => todo!(),
        }
    }
}

impl<'s> From<&'s [u8]> for Token<'s> {
    fn from(value: &'s [u8]) -> Self {
        match value {
            b"let" => Token::Let,
            b"fn" => Token::Function,
            b"if" => Token::If,
            b"else" => Token::Else,
            b"return" => Token::Return,
            b"true" => Token::True,
            b"false" => Token::False,
            num_slice if value[0].is_ascii_digit() => {
                let literal = std::str::from_utf8(num_slice).unwrap();
                Token::Int(literal)
            }
            _ => {
                let literal = std::str::from_utf8(value).unwrap();
                Token::Ident(literal)
            }
        }
    }
}
