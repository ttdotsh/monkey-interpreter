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
    curr_token: Token<'p>,
    next_token: Token<'p>,
    pub errors: Vec<ParseError>,
}

impl<'p> Parser<'p> {
    pub fn new<'s: 'p>(src: &'s str) -> Parser<'p> {
        let mut parser = Parser {
            lexer: Lexer::new(src),
            curr_token: Default::default(),
            next_token: Default::default(),
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
        while !self.curr_token.is(&Token::CloseCurly) && !self.curr_token.is(&Token::Eof) {
            match self.parse_stmt() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => self.errors.push(e),
            }
            self.step();
        }
        Ast::from(statements)
    }

    fn step(&mut self) {
        self.curr_token = std::mem::take(&mut self.next_token);
        self.next_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if self.next_token.is(&expected_token) {
            self.step();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken)
        }
    }

    fn expect_ident(&mut self) -> Result<(), ParseError> {
        match self.next_token {
            Token::Ident(_) => {
                self.step();
                Ok(())
            }
            _ => Err(ParseError::ExpectedIdentifier),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let statement = match self.curr_token {
            Token::Let => {
                let (ident, val) = self.parse_let_stmt()?;
                Stmt::Let { ident, val }
            }
            Token::Return => {
                self.step();
                Stmt::Return(self.parse_expr(Precedence::Lowest)?)
            }
            _ => Stmt::Expression(self.parse_expr(Precedence::Lowest)?),
        };

        if self.next_token.is(&Token::Semicolon) {
            self.step();
        }

        Ok(statement)
    }

    fn parse_let_stmt(&mut self) -> Result<(String, Expr), ParseError> {
        self.expect_ident()?;
        let name = String::from(self.curr_token.literal());

        self.expect_next(Token::Assign)?;
        self.step();

        let value = self.parse_expr(Precedence::Lowest)?;

        Ok((name, value))
    }

    fn parse_expr(&mut self, prec: Precedence) -> Result<Expr, ParseError> {
        let mut expression = match self.curr_token {
            Token::Ident(s) => Ok(Expr::Ident(String::from(s))),
            Token::Int(s) => {
                let int_val = s.parse().map_err(|_| ParseError::ParseIntError)?;
                Ok(Expr::IntLiteral(int_val))
            }
            Token::True | Token::False => {
                Ok(Expr::BooleanLiteral(self.curr_token.is(&Token::True)))
            }
            Token::Bang | Token::Minus => self.parse_prefix_expr(),
            Token::OpenParen => self.parse_grouped_expr(),
            Token::If => self.parse_if_expr(),
            Token::Function => self.parse_func_literal_expr(),
            _ => Err(ParseError::ExpectedExpression),
        }?;

        while !self.curr_token.is(&Token::Semicolon) && prec < Precedence::from(&self.next_token) {
            self.step();
            expression = match self.curr_token {
                Token::OpenParen => self.parse_func_call_expr(expression),
                _ => self.parse_infix_expr(expression),
            }?;
        }

        Ok(expression)
    }

    fn parse_prefix_expr(&mut self) -> Result<Expr, ParseError> {
        let operator = Operator::try_from(&self.curr_token)?;
        self.step();

        Ok(Expr::Prefix(
            operator,
            Box::new(self.parse_expr(Precedence::Prefix)?),
        ))
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Result<Expr, ParseError> {
        let operator = Operator::try_from(&self.curr_token)?;
        let prec = Precedence::from(&self.curr_token);

        self.step();
        let right = self.parse_expr(prec)?;

        Ok(Expr::Infix(Box::new(left), operator, Box::new(right)))
    }

    fn parse_grouped_expr(&mut self) -> Result<Expr, ParseError> {
        self.step();
        let expression = self.parse_expr(Precedence::Lowest)?;
        self.expect_next(Token::CloseParen)?;

        Ok(expression)
    }

    fn parse_if_expr(&mut self) -> Result<Expr, ParseError> {
        self.expect_next(Token::OpenParen)?;
        self.step();
        let condition = self.parse_expr(Precedence::Lowest)?;

        self.expect_next(Token::CloseParen)?;
        self.expect_next(Token::OpenCurly)?;
        let block = self.parse();

        let alt = if self.next_token.is(&Token::Else) {
            self.step();
            self.expect_next(Token::OpenCurly)?;
            Some(self.parse())
        } else {
            None
        };

        Ok(Expr::If {
            check: Box::new(condition),
            block,
            alt,
        })
    }

    fn parse_func_literal_expr(&mut self) -> Result<Expr, ParseError> {
        self.expect_next(Token::OpenParen)?;
        let params = self.parse_func_params()?;

        self.expect_next(Token::OpenCurly)?;
        let body = self.parse();

        Ok(Expr::FuncLiteral { params, body })
    }

    fn parse_func_call_expr(&mut self, function: Expr) -> Result<Expr, ParseError> {
        Ok(Expr::Call {
            func_name: Box::new(function),
            args: self.parse_func_args()?,
        })
    }

    fn parse_func_params(&mut self) -> Result<Params, ParseError> {
        let mut params = Vec::new();
        let end_of_params = Token::CloseParen;
        if self.next_token.is(&end_of_params) {
            self.step();
        } else {
            self.expect_ident()?;
            while !self.curr_token.is(&end_of_params) {
                params.push(self.parse_expr(Precedence::Lowest)?);
                if self.next_token.is(&Token::Comma) {
                    self.step();
                    self.expect_ident()?;
                } else {
                    self.expect_next(Token::CloseParen)?;
                }
            }
        }
        Ok(Params::from(params))
    }

    fn parse_func_args(&mut self) -> Result<Args, ParseError> {
        let mut args = Vec::new();
        let end_of_args = Token::CloseParen;
        self.step();
        while !self.curr_token.is(&end_of_args) {
            args.push(self.parse_expr(Precedence::Lowest)?);
            if self.next_token.is(&Token::Comma) {
                self.step();
                self.step(); // step past the comma, to the start of the next expression
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
