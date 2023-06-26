use std::fmt::Display;

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

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let { name, value } => write!(f, "let {} = {};", name, value),
            Self::Return(expr) => write!(f, "return {};", expr),
            Self::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

// TODO: I don't like that this is randomly its own type, explore some other implementations
#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = &self
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        write!(f, "{}", string)
    }
}

/*
* Expressions
*/
#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    IntLiteral(i64),
    BooleanLiteral(bool),
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Block,
        alternative: Option<Block>,
    },
    FuncLiteral {
        parameters: Parameters,
        body: Block,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(s) => write!(f, "{}", s),
            Self::IntLiteral(i) => write!(f, "{}", i),
            Self::BooleanLiteral(b) => write!(f, "{}", b),
            Self::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Self::Infix {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Parameters(pub Vec<Expression>);

impl Display for Parameters {
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
