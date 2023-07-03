use crate::token::Token;

/*
* Lexer
*/
pub struct Lexer<'a> {
    source: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.as_bytes(),
            position: 0,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_char() {
            Some(_) => Some(self.next_token()),
            None => None,
        }
    }
}

impl Lexer<'_> {
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.curr_char() {
            Some(b',') => Token::Comma,
            Some(b';') => Token::Semicolon,
            Some(b'(') => Token::OpenParen,
            Some(b')') => Token::CloseParen,
            Some(b'{') => Token::OpenCurly,
            Some(b'}') => Token::CloseCurly,
            Some(b'+') => Token::Plus,
            Some(b'-') => Token::Minus,
            Some(b'*') => Token::Asterisk,
            Some(b'/') => Token::Slash,
            Some(b'<') => Token::LessThan,
            Some(b'>') => Token::GreaterThan,

            Some(b'=') => match self.peek_char() {
                Some(b'=') => {
                    self.step();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            Some(b'!') => match self.peek_char() {
                Some(b'=') => {
                    self.step();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },

            Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                let ident_slice = self.read_identifier();
                // return Token::from(ident_slice);
                let token_literal = String::from_utf8_lossy(ident_slice).to_string();
                return Token::from(token_literal);
            }
            Some(b'0'..=b'9') => {
                let num_slice = self.read_number();
                let token_literal = String::from_utf8_lossy(num_slice).to_string();
                return Token::Int(token_literal);
            }

            None => Token::Eof,
            _ => Token::Illegal,
        };
        self.step();
        token
    }

    fn step(&mut self) {
        self.position += 1;
    }

    fn curr_char(&self) -> Option<&u8> {
        if self.position >= self.source.len() {
            None
        } else {
            Some(&self.source[self.position])
        }
    }

    fn peek_char(&self) -> Option<&u8> {
        let peek_pos = self.position + 1;
        if peek_pos >= self.source.len() {
            None
        } else {
            Some(&self.source[peek_pos])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&b' ' | &b'\t' | &b'\n' | &b'\r') = self.curr_char() {
            self.step();
        }
    }

    fn read_identifier(&mut self) -> &[u8] {
        let pos = self.position;
        while let Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') = self.curr_char() {
            self.step();
        }
        &self.source[pos..self.position]
    }

    fn read_number(&mut self) -> &[u8] {
        let pos = self.position;
        while let Some(b'0'..=b'9') = self.curr_char() {
            self.step();
        }
        &self.source[pos..self.position]
    }
}

/*
* Token impl for Lexer
*/
impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lex::Lexer, token::Token};

    #[test]
    fn test_next_token() {
        let test_input = "=+(){},;";
        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenCurly,
            Token::CloseCurly,
            Token::Comma,
            Token::Semicolon,
        ];
        let mut lexer = Lexer::new(test_input);
        for exp_tok in expected_tokens {
            let tok = lexer.next_token();
            assert_eq!(exp_tok, tok);
        }
    }

    #[test]
    fn test_syntax() {
        let test_input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                 x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10; 
            10 != 9;
        "#;
        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::OpenParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::CloseParen,
            Token::OpenCurly,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::CloseCurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::OpenParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::CloseParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreaterThan,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::OpenParen,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::CloseParen,
            Token::OpenCurly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::CloseCurly,
            Token::Else,
            Token::OpenCurly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::CloseCurly,
            Token::Int(String::from("10")),
            Token::Equal,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEqual,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(test_input);
        for exp_tok in expected_tokens {
            let tok = lexer.next_token();
            assert_eq!(exp_tok, tok);
        }
    }
}
