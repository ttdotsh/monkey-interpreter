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
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
}

impl Is for Statement {
    fn is(&self, subject: &Self) -> bool {
        match (self, subject) {
            (Statement::Let { .. }, Statement::Let { .. }) => true,
            (Statement::Return(_), Statement::Return(_)) => true,
            (Statement::Expression(_), Statement::Expression(_)) => true,
            _ => false,
        }
    }
}

/*
* Expressions
*/
pub enum Expression {
    Ident(String),
}
