use std::{fmt::Display, ops::Deref};

/*
* Abstract Syntax Tree
*/
#[derive(Debug, PartialEq)]
pub struct Ast(pub Vec<Stmt>);

pub type Block = Ast;

impl From<Vec<Stmt>> for Ast {
    fn from(value: Vec<Stmt>) -> Self {
        Ast(value)
    }
}

impl Deref for Ast {
    type Target = Vec<Stmt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Ast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", string)
    }
}

/*
* Statements
*/
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let { ident: String, val: Expr },
    Return(Expr),
    Expression(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let { ident, val } => write!(f, "let {} = {};", ident, val),
            Self::Return(expr) => write!(f, "return {};", expr),
            Self::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

/*
* Expressions
*/
#[derive(Debug, PartialEq)]
pub enum Expr {
    Ident(String),
    IntLiteral(i32),
    BooleanLiteral(bool),
    Prefix(Operator, Box<Expr>),
    Infix(Box<Expr>, Operator, Box<Expr>),
    If {
        condition: Box<Expr>,
        consequence: Block,
        alternative: Option<Block>,
    },
    FuncLiteral {
        parameters: Params,
        body: Block,
    },
    Call {
        func_name: Box<Expr>,
        arguments: Args,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(i) => write!(f, "{}", i),
            Self::IntLiteral(i) => write!(f, "{}", i),
            Self::BooleanLiteral(b) => write!(f, "{}", b),
            Self::Prefix(operator, right) => write!(f, "({}{})", operator, right),
            Self::Infix(left, operator, right) => write!(f, "({} {} {})", left, operator, right),
            Self::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, " else {}", alt)?;
                }
                Ok(())
            }
            Self::FuncLiteral { parameters, body } => {
                write!(f, "fn({}) {{ {} }}", parameters, body)
            }
            Self::Call {
                func_name,
                arguments,
            } => {
                write!(f, "{}({})", func_name, arguments)
            }
        }
    }
}

/*
* Function Parameters and Arguments
*/
#[derive(Debug, PartialEq)]
pub struct ExpressionList(Vec<Expr>);

pub type Params = ExpressionList;
pub type Args = ExpressionList;

impl From<Vec<Expr>> for ExpressionList {
    fn from(value: Vec<Expr>) -> Self {
        ExpressionList(value)
    }
}

impl Deref for ExpressionList {
    type Target = Vec<Expr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ExpressionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", string)
    }
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiplication => write!(f, "*"),
            Self::Division => write!(f, "/"),
            Self::GreaterThan => write!(f, ">"),
            Self::LessThan => write!(f, "<"),
            Self::Equals => write!(f, "=="),
            Self::NotEquals => write!(f, "!="),
        }
    }
}
