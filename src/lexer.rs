#[cfg(test)]
mod test {
    use crate::token::Token;

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
        let lexer = Lexer::new(test_input);
        for exp_tok in expected_tokens.iter() {
            let tok = lexer.next();
            assert_eq!(exp_tok, tok);
        }
    }
}
