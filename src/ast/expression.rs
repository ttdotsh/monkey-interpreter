use super::Node;
use crate::token::Token;

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

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);
