use crate::token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

#[allow(dead_code)]
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            // Delimiters
            b',' => Token::Comma,
            b';' => Token::Semicolon,

            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b'{' => Token::OpenCurly,
            b'}' => Token::CloseCurly,

            // Operators
            b'=' => Token::Assign,
            b'+' => Token::Plus,

            // TODO: Handle rest of cases
            _ => Token::Eof,
        };
        self.read_char();
        return tok;
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, token::Token};

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
        let mut lexer = Lexer::new(String::from(test_input));
        for exp_tok in expected_tokens.into_iter() {
            let tok = lexer.next_token();
            assert_eq!(exp_tok, tok);
        }
    }
}
