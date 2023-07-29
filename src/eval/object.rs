use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

#[derive(Debug, Clone)]
pub enum Object {
    /* Types */
    Integer(i32),
    Boolean(bool),

    ReturnValue(Box<Object>),
    Error(String),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::ReturnValue(v) => write!(f, "{}", v),
            Object::Error(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }
}

/*
* Prefix Operator Traits
*/
impl Not for Object {
    type Output = Self;

    fn not(self) -> Self::Output {
        Object::Boolean(!self.is_truthy())
    }
}

impl Neg for Object {
    type Output = Result<Self, String>;

    fn neg(self) -> Self::Output {
        match self {
            Object::Integer(i) => Ok(Object::Integer(-i)),
            _ => Err(format!("No such negative value of {}", self)),
        }
    }
}

/*
 * Infix Operator Traits
 */
impl Add for Object {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
            (l, r) => Err(format!("Cannot add {} to {}", l, r)),
        }
    }
}

impl Sub for Object {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
            (l, r) => Err(format!("Cannot subtract {} from {}", l, r)),
        }
    }
}

impl Mul for Object {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
            (l, r) => Err(format!("Cannot multiply {} and {}", l, r)),
        }
    }
}

impl Div for Object {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
            (l, r) => Err(format!("Cannot divide {} and {}", l, r)),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(l), Object::Integer(r)) => l == r,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::Error(l), Object::Error(r)) => l == r,
            (Object::ReturnValue(l), Object::ReturnValue(r)) => l == r,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Integer(l), Object::Integer(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}
