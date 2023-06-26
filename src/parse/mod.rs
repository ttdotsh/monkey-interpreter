#[cfg(test)]
mod test;

use crate::{
    ast::{Block, Expression, Operator, Parameters, Program, Statement},
    lex::Lexer,
    token::Token,
};

/*
* Parser
*/
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

    fn expect_ident(&mut self) -> Result<(), ParseError> {
        if self.peek_token.is_ident() {
            self.step();
            Ok(())
        } else {
            Err(ParseError::ExpectedIdentifier)
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
        // let expected_ident = Token::Ident(String::from("/* Variable Name */"));
        // self.expect_next(expected_ident)?;
        self.expect_ident()?;
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

    // TODO: Error on empty block
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
            Token::Function => self.parse_function_literal_expression(),
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

    fn parse_function_literal_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(Token::OpenParen)?;
        let parameters = self.parse_function_parameters()?;
        self.expect_next(Token::OpenCurly)?;
        let body = self.parse_block_statement();

        return Ok(Expression::FuncLiteral { parameters, body });
    }

    fn parse_function_parameters(&mut self) -> Result<Parameters, ParseError> {
        let mut parameters = Vec::new();
        if self.peek_token.is(&Token::CloseParen) {
            self.step();
            return Ok(Parameters(parameters));
        }

        self.expect_ident()?;
        while self.current_token.is_ident() {
            parameters.push(self.parse_expression(Precedence::Lowest)?);
            if self.peek_token.is(&Token::Comma) {
                self.step();
                self.expect_ident()?;
            } else {
                self.expect_next(Token::CloseParen)?;
            }
        }
        return Ok(Parameters(parameters));
    }
}

/*
* Precedence
*/
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

/*
* Token impls for Parser
*/
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

/*
* ParseError
*/
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: Token, recieved: Token },
    NoneTypeLiteral,
    ExpectedExpression,
    ParseIntError(String),
    ExpectedOperator,
    ExpectedIdentifier,
}
