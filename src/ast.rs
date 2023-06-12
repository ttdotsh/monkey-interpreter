use crate::is::Is;

/*
* Abstract Syntax Tree
*/
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        return Program {
            statements: Vec::new(),
        };
    }
}

/*
* Statements
*/
pub enum Statement {
    Let(Let),
    Return(Return),
}

pub struct Let {
    pub name: Identifier,
    pub value: Expression,
}

pub struct Return(pub Expression);

impl Is for Statement {
    fn is(&self, subject: &Self) -> bool {
        match (self, subject) {
            (Statement::Let(_), Statement::Let(_)) => true,
            (Statement::Return(_), Statement::Return(_)) => true,
            _ => false,
        }
    }
}

/*
* Expressions
*/
pub enum Expression {
    Ident(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);
