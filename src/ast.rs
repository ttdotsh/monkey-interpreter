use std::{fmt::Display, ops::Deref};

/*
* Abstract Syntax Tree
*/
#[derive(Debug, PartialEq, Clone)]
pub struct Ast(pub Vec<Stmt>);

impl From<Vec<Stmt>> for Ast {
    fn from(value: Vec<Stmt>) -> Self {
        Ast(value)
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
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
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(String),
    IntLiteral(i32),
    BooleanLiteral(bool),
    Prefix(Operator, Box<Expr>),
    Infix(Box<Expr>, Operator, Box<Expr>),
    If {
        check: Box<Expr>,
        block: Ast,
        alt: Option<Ast>,
    },
    FuncLiteral {
        params: Params,
        body: Ast,
    },
    Call {
        func: Box<Expr>,
        args: Args,
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
            Self::If { check, block, alt } => {
                write!(f, "if {} {}", check, block)?;
                if let Some(alt) = alt {
                    write!(f, " else {}", alt)?;
                }
                Ok(())
            }
            Self::FuncLiteral { params, body } => {
                write!(f, "fn({}) {{ {} }}", params, body)
            }
            Self::Call { func, args } => {
                write!(f, "{}({})", func, args)
            }
        }
    }
}

/*
* Function Parameters and Arguments
*/
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList(Vec<Expr>);

pub type Params = ExpressionList;
pub type Args = ExpressionList;

impl From<Vec<Expr>> for ExpressionList {
    fn from(value: Vec<Expr>) -> Self {
        ExpressionList(value)
    }
}

impl Display for ExpressionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", string)
    }
}

impl Deref for ExpressionList {
    type Target = Vec<Expr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for ExpressionList {
    type Item = Expr;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/*
* Operators
*/
#[derive(Debug, PartialEq, Clone)]
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
