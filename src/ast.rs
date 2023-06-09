use crate::token::Token;

trait Node {
    fn token(&self) -> Token;
}

#[allow(dead_code)]
pub enum Statement {
    Let(Let),
}

impl Node for Statement {
    fn token(&self) -> Token {
        match self {
            Statement::Let(_) => Token::Let,
        }
    }
}

#[allow(dead_code)]
pub struct Let {
    name: Identifier,
    value: Box<Expression>,
}

#[allow(dead_code)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token(&self) -> Token {
        match self {
            Expression::Identifier(Identifier(s)) => Token::Ident(s.to_owned()),
        }
    }
}

pub struct Identifier(String);

#[allow(dead_code)]
pub struct Program {
    statements: Vec<Statement>,
}
