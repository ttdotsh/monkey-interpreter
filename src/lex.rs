use crate::token::Token;

pub struct Lexer<'l> {
    src: &'l [u8],
    position: usize,
    ch: Option<u8>,
}

impl<'l> Lexer<'l> {
    pub fn new(source_code: &'l str) -> Lexer<'l> {
        let src = source_code.as_bytes();
        Lexer {
            src,
            position: 0,
            ch: Some(src[0]),
        }
    }

    pub fn next_token(&mut self) -> Token<'l> {
        self.skip_whitespace();
        let token = match self.ch {
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

            Some(b'=') => match self.peek() {
                Some(b'=') => {
                    self.step();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            Some(b'!') => match self.peek() {
                Some(b'=') => {
                    self.step();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },

            Some(b'0'..=b'9') => {
                return Token::from(self.read_num());
            }
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                return Token::from(self.read_ident());
            }

            None => Token::Eof,
            _ => Token::Illegal,
        };
        self.step();
        token
    }

    fn step(&mut self) {
        self.position += 1;
        if self.position >= self.src.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.src[self.position])
        }
    }

    fn peek(&self) -> Option<u8> {
        let peek_pos = self.position + 1;
        if peek_pos >= self.src.len() {
            None
        } else {
            Some(self.src[peek_pos])
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                Some(b' ' | b'\t' | b'\n' | b'\r') => self.step(),
                _ => break,
            }
        }
    }

    fn read_ident(&mut self) -> &'l str {
        let pos = self.position;
        loop {
            match self.ch {
                Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => self.step(),
                _ => break,
            }
        }
        let slice = &self.src[pos..self.position];
        let literal = unsafe { std::str::from_utf8_unchecked(slice) };
        literal
    }

    fn read_num(&mut self) -> &'l str {
        let pos = self.position;
        loop {
            match self.ch {
                Some(b'0'..=b'9') => self.step(),
                _ => break,
            }
        }
        let slice = &self.src[pos..self.position];
        let literal = unsafe { std::str::from_utf8_unchecked(slice) };
        literal
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
        expected_tokens
            .into_iter()
            .for_each(|t| assert_eq!(t, lexer.next_token()));
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
            Token::Ident("five"),
            Token::Assign,
            Token::Int("5"),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten"),
            Token::Assign,
            Token::Int("10"),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add"),
            Token::Assign,
            Token::Function,
            Token::OpenParen,
            Token::Ident("x"),
            Token::Comma,
            Token::Ident("y"),
            Token::CloseParen,
            Token::OpenCurly,
            Token::Ident("x"),
            Token::Plus,
            Token::Ident("y"),
            Token::Semicolon,
            Token::CloseCurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result"),
            Token::Assign,
            Token::Ident("add"),
            Token::OpenParen,
            Token::Ident("five"),
            Token::Comma,
            Token::Ident("ten"),
            Token::CloseParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5"),
            Token::Semicolon,
            Token::Int("5"),
            Token::LessThan,
            Token::Int("10"),
            Token::GreaterThan,
            Token::Int("5"),
            Token::Semicolon,
            Token::If,
            Token::OpenParen,
            Token::Int("5"),
            Token::LessThan,
            Token::Int("10"),
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
            Token::Int("10"),
            Token::Equal,
            Token::Int("10"),
            Token::Semicolon,
            Token::Int("10"),
            Token::NotEqual,
            Token::Int("9"),
            Token::Semicolon,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(test_input);
        expected_tokens
            .into_iter()
            .for_each(|t| assert_eq!(t, lexer.next_token()));
    }
}
