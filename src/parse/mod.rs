#[cfg(test)]
mod test;

use crate::{
    ast::{Arguments, Ast, Block, Expression, Indentifier, Operator, Parameters, Statement},
    lex::Lexer,
    token::Token,
};

/*
* Parser
*/
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<ParseError>,
}

impl Parser<'_> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut program = Vec::new();
        self.step();
        self.step();
        while !self.current_token.is(&Token::Eof) {
            match self.parse_statement() {
                Ok(s) => program.push(s),
                Err(e) => self.errors.push(e),
            }
            self.step();
        }
        Ast::from(program)
    }

    fn step(&mut self) {
        self.current_token = std::mem::take(&mut self.peek_token);
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

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token {
            Token::Let => {
                let (name, value) = self.parse_let_statement()?;
                Ok(Statement::Let { name, value })
            }
            Token::Return => Ok(Statement::Return(self.parse_return_statement()?)),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<(Indentifier, Expression), ParseError> {
        let name = match &mut self.peek_token {
            Token::Ident(s) => {
                let literal = std::mem::take(s);
                self.step();
                Ok(Indentifier::from(literal))
            }
            _ => Err(ParseError::ExpectedIdentifier),
        }?;

        self.expect_next(Token::Assign)?;
        self.step();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }

        Ok((name, value))
    }

    fn parse_return_statement(&mut self) -> Result<Expression, ParseError> {
        self.step();

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }

        Ok(return_value)
    }

    fn parse_expression_statement(&mut self) -> Result<Expression, ParseError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }
        Ok(expression)
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut statements = Vec::new();
        self.step();

        while !self.current_token.is(&Token::CloseCurly) && !self.current_token.is(&Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.step();
        }

        if statements.is_empty() {
            Err(ParseError::ExpectedStatement)
        } else {
            Ok(Block(statements))
        }
    }

    fn parse_expression(&mut self, cur_precedence: Precedence) -> Result<Expression, ParseError> {
        let mut expression = match &mut self.current_token {
            Token::Ident(s) => Ok(Expression::Ident(std::mem::take(s).into())),
            Token::Int(s) => {
                let int_literal = s
                    .parse()
                    .map_err(|_| ParseError::ParseIntError(std::mem::take(s)))?;
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
            expression = match self.current_token {
                Token::OpenParen => self.parse_function_call_expression(expression),
                _ => self.parse_infix_expression(expression),
            }?;
        }

        Ok(expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let operator = Operator::try_from(&self.current_token)?;
        self.step();

        Ok(Expression::Prefix {
            operator,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let precedence = self.current_token.precedence();
        let operator = Operator::try_from(&self.current_token)?;

        self.step();
        let right = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.step();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_next(Token::CloseParen)?;

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(Token::OpenParen)?;
        self.step();
        let condition = self.parse_expression(Precedence::Lowest)?;

        self.expect_next(Token::CloseParen)?;
        self.expect_next(Token::OpenCurly)?;
        let consequence = self.parse_block()?;

        let alternative = if self.peek_token.is(&Token::Else) {
            self.step();
            self.expect_next(Token::OpenCurly)?;
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_function_literal_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(Token::OpenParen)?;
        let parameters = self.parse_function_parameters()?;

        self.expect_next(Token::OpenCurly)?;
        let body = self.parse_block()?;

        Ok(Expression::FuncLiteral { parameters, body })
    }

    fn parse_function_call_expression(
        &mut self,
        function: Expression,
    ) -> Result<Expression, ParseError> {
        let arguments = self.parse_function_call_arguments()?;

        Ok(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Parameters, ParseError> {
        let mut parameters = Vec::new();
        let end_of_params = Token::CloseParen;
        if self.peek_token.is(&end_of_params) {
            self.step();
            Ok(Parameters(parameters))
        } else {
            self.expect_ident()?;
            while !self.current_token.is(&end_of_params) {
                parameters.push(self.parse_expression(Precedence::Lowest)?);
                if self.peek_token.is(&Token::Comma) {
                    self.step();
                    self.expect_ident()?;
                } else {
                    self.expect_next(Token::CloseParen)?;
                }
            }
            Ok(Parameters(parameters))
        }
    }

    fn parse_function_call_arguments(&mut self) -> Result<Arguments, ParseError> {
        let mut args = Vec::new();
        let end_of_args = Token::CloseParen;
        self.step();
        while !self.current_token.is(&end_of_args) {
            args.push(self.parse_expression(Precedence::Lowest)?);
            if self.peek_token.is(&Token::Comma) {
                self.step();
                self.step();
            } else {
                self.expect_next(Token::CloseParen)?;
            }
        }
        Ok(Arguments(args))
    }
}

/*
* Precedence
*/
#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 1,
    Equality = 2,    /*     == or !=     */
    LessGreater = 3, /*      < or >      */
    AddSub = 4,      /*      + or -      */
    MultDiv = 5,     /*      * or /      */
    Prefix = 6,      /*     -x or !x     */
    Call = 7,        /*  my_function(x)  */
}

/*
* Token impls for Parser
*/
impl Token {
    fn precedence(&self) -> Precedence {
        match self {
            Token::OpenParen => Precedence::Call,
            Token::Asterisk | Token::Slash => Precedence::MultDiv,
            Token::Plus | Token::Minus => Precedence::AddSub,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Equal | Token::NotEqual => Precedence::Equality,
            _ => Precedence::Lowest,
        }
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
    ExpectedExpression,
    ParseIntError(String),
    ExpectedOperator,
    ExpectedIdentifier,
    ExpectedStatement,
}
