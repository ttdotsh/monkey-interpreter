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
}

pub struct Let {
    pub name: Identifier,
    pub value: Expression,
}

/*
* Expressions
*/
pub enum Expression {
    Ident(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);
