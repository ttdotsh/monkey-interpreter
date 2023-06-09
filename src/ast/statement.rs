use super::{
    expression::{Expression, Identifier},
    Node,
};
use crate::token::Token;

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
    pub name: Identifier,
    pub value: Box<Expression>,
}
