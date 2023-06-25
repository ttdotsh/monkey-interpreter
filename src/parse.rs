use crate::{
    ast::{Block, Expression, Operator, Program, Statement},
    lex::Lexer,
    token::Token,
};

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
        while !self.current_token.is(&Token::Eof) {
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

        // TODO: implement parsing expressions");
        while self.current_token != Token::Semicolon {
            self.step();
        }
        let value = String::from("value");

        return Ok((name, Expression::Ident(value)));
    }

    fn parse_return_statement(&mut self) -> Result<Expression, ParseError> {
        self.step();

        // TODO: implement parsing expressions");
        while self.current_token != Token::Semicolon {
            self.step();
        }
        let value = String::from("value");

        return Ok(Expression::Ident(value));
    }

    fn parse_expression_statement(&mut self) -> Result<Expression, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }
        return Ok(expression);
    }

    fn parse_expression(&mut self, cur_precedence: Precedence) -> Result<Expression, ParseError> {
        let mut expression = match &self.current_token {
            Token::Ident(s) => Ok(Expression::Ident(s.to_owned())), // TODO: move s out of token?
            Token::Int(s) => {
                // TODO: move the string rather than copy?
                let int_literal = s.parse().map_err(|_| ParseError::ParseIntError(s.into()))?;
                Ok(Expression::IntLiteral(int_literal))
            }
            Token::True | Token::False => Ok(Expression::BooleanLiteral(
                self.current_token.is(&Token::True),
            )),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            Token::OpenParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            _ => Err(ParseError::ExpectedExpression),
        }?;

        while !self.current_token.is(&Token::Semicolon)
            && cur_precedence < self.peek_token.precedence()
        {
            self.step();
            expression = self.parse_infix_expression(expression)?;
        }

        return Ok(expression);
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let operator = Operator::try_from(&self.current_token)?;
        self.step();

        return Ok(Expression::Prefix {
            operator,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        });
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let precedence = self.current_token.precedence();
        let operator = Operator::try_from(&self.current_token)?;

        self.step();
        let right = self.parse_expression(precedence)?;

        return Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        });
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.step();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_next(Token::CloseParen)?;
        return Ok(expression);
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(Token::OpenParen)?;
        self.step();
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_next(Token::CloseParen)?;
        self.expect_next(Token::OpenCurly)?;
        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token.is(&Token::Else) {
            self.step();
            self.expect_next(Token::OpenCurly)?;
            Some(self.parse_block_statement())
        } else {
            None
        };

        return Ok(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        });
    }

    fn parse_block_statement(&mut self) -> Block {
        let mut statements = Vec::new();
        self.step();

        while !self.current_token.is(&Token::CloseCurly) && !self.current_token.is(&Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.step();
        }

        return Block(statements);
    }
}

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 1,
    Equality = 2,    // == or !=
    LessGreater = 3, // < or >
    AddSub = 4,      // + or -
    MultDiv = 5,     // * or /
    Prefix = 6,      // -x or !x
    Call = 7,        // my_function(x)
}

impl Token {
    fn precedence(&self) -> Precedence {
        return match self {
            Token::Asterisk | Token::Slash => Precedence::MultDiv,
            Token::Plus | Token::Minus => Precedence::AddSub,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Equal | Token::NotEqual => Precedence::Equality,
            _ => Precedence::Lowest,
        };
    }
}

impl TryFrom<&Token> for Operator {
    type Error = ParseError;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Equal => Ok(Operator::Equals),
            Token::NotEqual => Ok(Operator::NotEquals),
            Token::LessThan => Ok(Operator::LessThan),
            Token::GreaterThan => Ok(Operator::GreaterThan),
            Token::Plus => Ok(Operator::Plus),
            Token::Minus => Ok(Operator::Minus),
            Token::Asterisk => Ok(Operator::Multiplication),
            Token::Slash => Ok(Operator::Division),
            Token::Bang => Ok(Operator::Bang),
            _ => Err(Self::Error::ExpectedOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: Token, recieved: Token },
    NoneTypeLiteral,
    ExpectedExpression,
    ParseIntError(String),
    ExpectedOperator,
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Block, Expression, Operator, Statement},
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
                    // TODO: add expected expressions here
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
    fn test_parse_boolean_literal_expression() {
        let test_input = r#"
            true;
            false;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let expected_statements = vec![
            Statement::Expression(Expression::BooleanLiteral(true)),
            Statement::Expression(Expression::BooleanLiteral(false)),
        ];

        expected_statements
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
    }

    #[test]
    fn test_parse_prefix_expression() {
        let test_input = r#"
            !5;
            -15;
            !true;
            !false;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);

        let expected_statements = vec![
            Statement::Expression(Expression::Prefix {
                operator: Operator::Bang,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Prefix {
                operator: Operator::Minus,
                right: Box::new(Expression::IntLiteral(15)),
            }),
            Statement::Expression(Expression::Prefix {
                operator: Operator::Bang,
                right: Box::new(Expression::BooleanLiteral(true)),
            }),
            Statement::Expression(Expression::Prefix {
                operator: Operator::Bang,
                right: Box::new(Expression::BooleanLiteral(false)),
            }),
        ];

        expected_statements
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
    }

    #[test]
    fn test_parse_infix_expression() {
        let test_input = r#"
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;
            true == true;
            true != false;
            false == false;
       "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors.len(), 0);

        let expected_statements = vec![
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::Minus,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::Multiplication,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::Division,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::GreaterThan,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::LessThan,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::Equals,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Operator::NotEquals,
                right: Box::new(Expression::IntLiteral(5)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::BooleanLiteral(true)),
                operator: Operator::Equals,
                right: Box::new(Expression::BooleanLiteral(true)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::BooleanLiteral(true)),
                operator: Operator::NotEquals,
                right: Box::new(Expression::BooleanLiteral(false)),
            }),
            Statement::Expression(Expression::Infix {
                left: Box::new(Expression::BooleanLiteral(false)),
                operator: Operator::Equals,
                right: Box::new(Expression::BooleanLiteral(false)),
            }),
        ];

        expected_statements
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
    }

    #[test]
    fn test_operator_precedence_parsing() -> std::fmt::Result {
        // These were copied from the book
        let expressions_and_expectations = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for (expr, expect) in expressions_and_expectations {
            let lexer = Lexer::new(expr.into());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            let ast_string = program
                .statements
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(ast_string, expect);
        }
        Ok(())
    }

    #[test]
    fn test_if_expression() {
        let test_input = r#"
            if (x < y) { x }
            if (x < y) { x } else { y }
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let expected_statements = vec![
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix {
                    left: Box::new(Expression::Ident(String::from("x"))),
                    operator: Operator::LessThan,
                    right: Box::new(Expression::Ident(String::from("y"))),
                }),
                consequence: Block(vec![Statement::Expression(Expression::Ident(
                    String::from("x"),
                ))]),
                alternative: None,
            }),
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix {
                    left: Box::new(Expression::Ident(String::from("x"))),
                    operator: Operator::LessThan,
                    right: Box::new(Expression::Ident(String::from("y"))),
                }),
                consequence: Block(vec![Statement::Expression(Expression::Ident(
                    String::from("x"),
                ))]),
                alternative: Some(Block(vec![Statement::Expression(Expression::Ident(
                    String::from("y"),
                ))])),
            }),
        ];

        expected_statements
            .into_iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
    }
}
