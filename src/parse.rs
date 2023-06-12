use crate::{
    ast::{Expression, Identifier, Let, Program, Return, Statement},
    is::Is,
    lex::Lexer,
    token::Token,
};

#[derive(Debug, PartialEq)]
enum ParseError {
    UnexpectedToken { expected: Token, recieved: Token },
    NoneTypeLiteral,
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

#[allow(dead_code)]
impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        return Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        };
    }

    fn step(&mut self) {
        std::mem::swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if self.peek_token.is(&expected_token) {
            self.step();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: expected_token,
                recieved: self.peek_token.to_owned(),
            })
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        while self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.step();
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => match self.parse_let_statement() {
                Ok(statement) => Some(Statement::Let(statement)),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            },
            Token::Return => match self.parse_return_statement() {
                Ok(statement) => Some(Statement::Return(statement)),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            },
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Result<Let, ParseError> {
        let expected_ident = Token::Ident(String::from("/* Variable Name */"));
        self.expect_next(expected_ident)?;
        let name = match self.current_token.extract_literal() {
            Some(s) => s,
            None => return Err(ParseError::NoneTypeLiteral), // I don't think this is possible?
        };

        self.expect_next(Token::Assign)?;

        // todo!("implement parsing expressions");
        while self.current_token != Token::Semicolon {
            self.step();
        }
        let value = String::from("value");

        return Ok(Let {
            name: Identifier(name),
            value: Expression::Ident(Identifier(value)),
        });
    }

    fn parse_return_statement(&mut self) -> Result<Return, ParseError> {
        self.step();

        // todo!("implement parsing expressions");
        while self.current_token != Token::Semicolon {
            self.step();
        }
        let value = String::from("value");

        return Ok(Return(Expression::Ident(Identifier(value))));
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, Identifier, Return, Statement},
        is::Is,
        lex::Lexer,
        parse::{ParseError, Parser},
        token::Token,
    };

    #[test]
    fn test_parse_let_statements() {
        let test_input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        let expected_indents = vec![
            Identifier(String::from("x")),
            Identifier(String::from("y")),
            Identifier(String::from("foobar")),
        ];

        for (i, statement) in program.statements.into_iter().enumerate() {
            if let Statement::Let(ls) = statement {
                assert_eq!(expected_indents[i], ls.name);
                // todo!("add expected expressions here");
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let test_input = r#"
            return 5;
            return 10;
            return 993322;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 3);

        // TODO: this is gross 👇
        let expected_statement_type = Statement::Return(Return(Expression::Ident(Identifier(
            String::from("/* Value */"),
        ))));
        for statement in program.statements {
            assert!(statement.is(&expected_statement_type));
        }
    }

    #[test]
    fn test_let_statement_syntax_errors() {
        let test_input = r#"
            let = 5;
            let y y 10;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 0);

        let expected_errors = vec![
            ParseError::UnexpectedToken {
                expected: Token::Ident(String::from("/* Variable Name */")),
                recieved: Token::Assign,
            },
            ParseError::UnexpectedToken {
                expected: Token::Assign,
                recieved: Token::Ident(String::from("y")),
            },
        ];
        for (i, error) in parser.errors.into_iter().enumerate() {
            assert_eq!(expected_errors[i], error);
        }
    }
}
