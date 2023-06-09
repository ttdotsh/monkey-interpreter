use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        return lex;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            b',' => Token::Comma,
            b';' => Token::Semicolon,

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b'{' => Token::OpenCurly,
            b'}' => Token::CloseCurly,

            b'=' => {
                if self.peek_next_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_next_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let token_literal = self.read_identifier();
                return Lexer::lookup_identifier(token_literal);
            }
            b'0'..=b'9' => {
                let token_literal = self.read_number();
                return Token::Int(token_literal);
            }

            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_next_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        return self.input[self.read_position];
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn lookup_identifier(token_literal: String) -> Token {
        match token_literal.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(token_literal),
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ch == 0 {
            return None;
        }
        return Some(self.next_token());
    }
}

#[cfg(test)]
mod test {
    use std::string::FromUtf8Error;

    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_next_token() -> Result<(), FromUtf8Error> {
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
        let mut lexer = Lexer::new(String::from(test_input));
        for exp_tok in expected_tokens.into_iter() {
            let tok = lexer.next_token();
            println!("Expected token: {:?}\nRecieved token: {:?}", exp_tok, tok);
            assert_eq!(exp_tok, tok);
        }
        Ok(())
    }

    #[test]
    fn test_syntax() -> Result<(), FromUtf8Error> {
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
        let mut lexer = Lexer::new(String::from(test_input));
        for exp_tok in expected_tokens.into_iter() {
            let tok = lexer.next_token();
            println!("Expected token: {:?}\nRecieved token: {:?}", exp_tok, tok);
            assert_eq!(exp_tok, tok);
        }
        Ok(())
    }
}
