use crate::{
    ast::{Expression, Operator, Program, Statement},
    lex::Lexer,
    token::Token,
};

#[derive(Debug, PartialEq)]
enum ParseError {
    UnexpectedToken { expected: Token, recieved: Token },
    NoneTypeLiteral,
    ExpectedExpression,
    ParseIntError(String),
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
                Ok((name, value)) => Some(Statement::Let { name, value }),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            },
            Token::Return => match self.parse_return_statement() {
                Ok(expr) => Some(Statement::Return(expr)),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            },
            _ => match self.parse_expression_statement() {
                Ok(expr) => Some(Statement::Expression(expr)),
                Err(e) => {
                    self.errors.push(e);
                    None
                }
            },
        }
    }

    fn parse_let_statement(&mut self) -> Result<(String, Expression), ParseError> {
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

        return Ok((name, Expression::Ident(value)));
    }

    fn parse_return_statement(&mut self) -> Result<Expression, ParseError> {
        self.step();

        // todo!("implement parsing expressions");
        while self.current_token != Token::Semicolon {
            self.step();
        }
        let value = String::from("value");

        return Ok(Expression::Ident(value));
    }

    fn parse_expression_statement(&mut self) -> Result<Expression, ParseError> {
        let expression = self.parse_expression()?;
        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }
        return Ok(expression);
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        match &self.current_token {
            Token::Ident(s) => Ok(Expression::Ident(s.to_owned())),
            Token::Int(s) => {
                // TODO: figure out if this should move the string rather than copy
                let int_literal = s.parse().map_err(|_| ParseError::ParseIntError(s.into()))?;
                Ok(Expression::IntLiteral(int_literal))
            }
            Token::Bang => {
                self.step();
                Ok(Expression::Prefix {
                    operator: Operator::Bang,
                    operand: Box::new(self.parse_expression()?),
                })
            }
            Token::Minus => {
                self.step();
                Ok(Expression::Prefix {
                    operator: Operator::Minus,
                    operand: Box::new(self.parse_expression()?),
                })
            }
            _ => Err(ParseError::ExpectedExpression),
        }
    }
}

#[allow(dead_code)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // *
    Prefix,      // -x or !x
    Call,        // my_function(x)
}

#[allow(dead_code)]
impl Precedence {
    fn value(&self) -> i32 {
        match self {
            Precedence::Lowest => 1,
            Precedence::Equals => 2,
            Precedence::LessGreater => 3,
            Precedence::Sum => 4,
            Precedence::Product => 5,
            Precedence::Prefix => 6,
            Precedence::Call => 7,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Expression, Operator, Statement},
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

        let expected_indents = vec![String::from("x"), String::from("y"), String::from("foobar")];

        for (i, statement) in program.statements.into_iter().enumerate() {
            match statement {
                Statement::Let { name, .. } => {
                    assert_eq!(expected_indents[i], name)
                    // todo!("add expected expressions here");
                }
                _ => assert!(false),
            };
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

        for statement in program.statements {
            match statement {
                Statement::Return(_) => {}
                _ => assert!(false),
            }
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
        let _program = parser.parse_program();

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

        for e in expected_errors {
            assert!(parser.errors.contains(&e));
        }
    }

    #[test]
    fn test_parse_identifier_expression() {
        let test_input = "foobar;";
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        let expected_statement = Statement::Expression(Expression::Ident(String::from("foobar")));
        assert_eq!(expected_statement, program.statements[0]);
    }

    #[test]
    fn test_parse_int_literal_expression() {
        let test_input = "5;";
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(program.statements.len(), 1);

        let expected_statement = Statement::Expression(Expression::IntLiteral(5));
        assert_eq!(expected_statement, program.statements[0]);
    }

    #[test]
    fn test_parse_prefix_expression() {
        let test_input = r#"
            !5;
            -15;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 2);

        let expected_expressions = vec![
            Statement::Expression(Expression::Prefix {
                operator: Operator::Bang,
                operand: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Prefix {
                operator: Operator::Minus,
                operand: Box::new(Expression::IntLiteral(15)),
            }),
        ];

        for (i, expr) in expected_expressions.into_iter().enumerate() {
            assert_eq!(expr, program.statements[i]);
        }
    }
}
