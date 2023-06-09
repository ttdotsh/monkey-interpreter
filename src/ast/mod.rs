pub mod expression;
pub mod statement;

use crate::token::Token;
use statement::Statement;

pub trait Node {
    fn token(&self) -> Token;
}

#[allow(dead_code)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[allow(dead_code)]
impl Program {
    pub fn new() -> Program {
        return Program {
            statements: Vec::new(),
        };
    }
}
