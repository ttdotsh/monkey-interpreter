#[derive(Debug, Default, PartialEq)]
pub enum Token<'a> {
    /* Identifiers and Literals */
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

impl<'t> From<&'t str> for Token<'t> {
    fn from(value: &'t str) -> Self {
        match value {
            "let" => Token::Let,
            "fn" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ if value.chars().all(|c| c.is_ascii_digit()) => Token::Int(value),
            _ => Token::Ident(value),
        }
    }
}
