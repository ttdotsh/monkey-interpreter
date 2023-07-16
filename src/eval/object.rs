use std::fmt::Display;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Object {
    /* Types */
    Integer(i32),
    Boolean(bool),

    ReturnValue(Box<Object>),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::ReturnValue(v) => write!(f, "{}", v),
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
