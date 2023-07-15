#[cfg(test)]
mod test;

use crate::{
    ast::{Args, Ast, Expr, Operator, Params, Stmt},
    lex::Lexer,
    token::Token,
};

/*
* Parser
*/
pub struct Parser<'p> {
    lexer: Lexer<'p>,
    current_token: Token<'p>,
    peek_token: Token<'p>,
    pub errors: Vec<ParseError>,
}

impl<'p> Parser<'p> {
    pub fn new<'s: 'p>(src: &'s str) -> Parser<'p> {
        let mut parser = Parser {
            lexer: Lexer::new(src),
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: Vec::new(),
        };
        parser.step();
        parser
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Ast {
        let mut statements = Vec::new();
        self.step();
        while !self.current_token.is(&Token::CloseCurly) && !self.current_token.is(&Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.step();
        }
        Ast::from(statements)
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
            Err(ParseError::UnexpectedToken)
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

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        let statement = match self.current_token {
            Token::Let => {
                let (ident, val) = self.parse_let_statement()?;
                Stmt::Let { ident, val }
            }
            Token::Return => {
                self.step();
                Stmt::Return(self.parse_expression(Precedence::Lowest)?)
            }
            _ => Stmt::Expression(self.parse_expression(Precedence::Lowest)?),
        };

        if self.peek_token.is(&Token::Semicolon) {
            self.step();
        }

        Ok(statement)
    }

    fn parse_let_statement(&mut self) -> Result<(String, Expr), ParseError> {
        self.expect_ident()?;
        let name = String::from(self.current_token.literal());

        self.expect_next(Token::Assign)?;
        self.step();

        let value = self.parse_expression(Precedence::Lowest)?;

        Ok((name, value))
    }

    fn parse_expression(&mut self, cur_precedence: Precedence) -> Result<Expr, ParseError> {
        let mut expression = match self.current_token {
            Token::Ident(s) => Ok(Expr::Ident(String::from(s))),
            Token::Int(s) => {
                let int_val = s.parse().map_err(|_| ParseError::ParseIntError)?;
                Ok(Expr::IntLiteral(int_val))
            }
            Token::True | Token::False => {
                Ok(Expr::BooleanLiteral(self.current_token.is(&Token::True)))
            }
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            Token::OpenParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal_expression(),
            _ => Err(ParseError::ExpectedExpression),
        }?;

        while !self.current_token.is(&Token::Semicolon)
            && cur_precedence < Precedence::from(&self.peek_token)
        {
            self.step();
            expression = match self.current_token {
                Token::OpenParen => self.parse_function_call_expression(expression),
                _ => self.parse_infix_expression(expression),
            }?;
        }

        Ok(expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expr, ParseError> {
        let operator = Operator::try_from(&self.current_token)?;
        self.step();

        Ok(Expr::Prefix(
            operator,
            Box::new(self.parse_expression(Precedence::Prefix)?),
        ))
    }

    fn parse_infix_expression(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let operator = Operator::try_from(&self.current_token)?;
        let prec = Precedence::from(&self.current_token);

        self.step();
        let right = self.parse_expression(prec)?;

        Ok(Expr::Infix(Box::new(left), operator, Box::new(right)))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expr, ParseError> {
        self.step();
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_next(Token::CloseParen)?;

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expr, ParseError> {
        self.expect_next(Token::OpenParen)?;
        self.step();
        let condition = self.parse_expression(Precedence::Lowest)?;

        self.expect_next(Token::CloseParen)?;
        self.expect_next(Token::OpenCurly)?;
        let consequence = self.parse();

        let alternative = if self.peek_token.is(&Token::Else) {
            self.step();
            self.expect_next(Token::OpenCurly)?;
            Some(self.parse())
        } else {
            None
        };

        Ok(Expr::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_function_literal_expression(&mut self) -> Result<Expr, ParseError> {
        self.expect_next(Token::OpenParen)?;
        let parameters = self.parse_function_params()?;

        self.expect_next(Token::OpenCurly)?;
        let body = self.parse();

        Ok(Expr::FuncLiteral { parameters, body })
    }

    fn parse_function_call_expression(&mut self, function: Expr) -> Result<Expr, ParseError> {
        Ok(Expr::Call {
            func_name: Box::new(function),
            arguments: self.parse_function_args()?,
        })
    }

    fn parse_function_params(&mut self) -> Result<Params, ParseError> {
        let mut params = Vec::new();
        let end_of_params = Token::CloseParen;
        if self.peek_token.is(&end_of_params) {
            self.step();
        } else {
            self.expect_ident()?;
            while !self.current_token.is(&end_of_params) {
                params.push(self.parse_expression(Precedence::Lowest)?);
                if self.peek_token.is(&Token::Comma) {
                    self.step();
                    self.expect_ident()?;
                } else {
                    self.expect_next(Token::CloseParen)?;
                }
            }
        }
        Ok(Params::from(params))
    }

    fn parse_function_args(&mut self) -> Result<Args, ParseError> {
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
        Ok(Args::from(args))
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
* Mapping Precedence and Operators to Tokens
*/
impl From<&Token<'_>> for Precedence {
    fn from(value: &Token) -> Self {
        match value {
            Token::OpenParen => Precedence::Call,
            Token::Asterisk | Token::Slash => Precedence::MultDiv,
            Token::Plus | Token::Minus => Precedence::AddSub,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Equal | Token::NotEqual => Precedence::Equality,
            _ => Precedence::Lowest,
        }
    }
}

impl TryFrom<&Token<'_>> for Operator {
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
    UnexpectedToken,
    ExpectedExpression,
    ParseIntError,
    ExpectedOperator,
    ExpectedIdentifier,
}
