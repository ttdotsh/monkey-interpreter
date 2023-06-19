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

/*
* Expressions
*/
pub enum Expression {
    Ident(String),
}
