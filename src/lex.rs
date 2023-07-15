use crate::token::Token;

/*
* Lexer
*/
pub struct Lexer<'l> {
    source: &'l [u8],
    position: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Lexer<'l> {
        Lexer {
            source: source.as_bytes(),
            position: 0,
        }
    }
}

impl<'l> Lexer<'l> {
    pub fn next_token(&mut self) -> Token<'l> {
        self.skip_whitespace();
        let token: Token<'l> = match self.curr_char() {
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

            /*
             * Early return here because these two methods advance the lexer past the last
             * character in the token, so we want to skip the next call to `self.step()`
             */
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') => return Token::from(self.read_identifier()),
            Some(b'0'..=b'9') => return Token::from(self.read_number()),

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

    fn peek(&self) -> Option<&u8> {
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

    fn read_identifier(&mut self) -> &'l [u8] {
        let pos = self.position;
        while let Some(b'a'..=b'z' | b'A'..=b'Z' | b'_') = self.curr_char() {
            self.step();
        }
        &self.source[pos..self.position]
    }

    fn read_number(&mut self) -> &'l [u8] {
        let pos = self.position;
        while let Some(b'0'..=b'9') = self.curr_char() {
            self.step();
        }
        &self.source[pos..self.position]
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
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::OpenParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::CloseParen,
            Token::OpenCurly,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::CloseCurly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::OpenParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::CloseParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::GreaterThan,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::OpenParen,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
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
            Token::Int("10".into()),
            Token::Equal,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEqual,
            Token::Int("9".into()),
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
