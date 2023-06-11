/*
* Abstract Syntax Tree
*/
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

/*
* Statements
*/
#[allow(dead_code)]
pub enum Statement {
    Let(Let),
}

#[allow(dead_code)]
pub struct Let {
    pub name: Identifier,
    pub value: Expression,
}

/*
* Expressions
*/
#[allow(dead_code)]
pub enum Expression {
    Ident(Identifier),
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);
