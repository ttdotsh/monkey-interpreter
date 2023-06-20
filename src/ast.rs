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
#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
}

/*
* Expressions
*/
#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    IntLiteral(i64),
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

/*
* Operators
*/
#[derive(Debug, PartialEq)]
pub enum Operator {
    Bang,
    Plus,
    Minus,
    Multiplication,
    Division,
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}
