use crate::token::Token;

trait Node {
    fn token(&self) -> Token;
}

trait Expression {
    // type Value;
    //
    // fn value() -> Self::Value;
}

struct Identifier(String);

impl Node for Identifier {
    fn token(&self) -> Token {
        Token::Ident(self.0.to_owned())
    }
}

impl Expression for Identifier {}

#[allow(dead_code)]
enum Statement {
    Let {
        name: Identifier,
        value: Box<dyn Expression>,
    },
}

impl Node for Statement {
    fn token(&self) -> Token {
        match self {
            Statement::Let { name: _, value: _ } => Token::Let,
        }
    }
}

#[allow(dead_code)]
struct Program {
    statements: Vec<Statement>,
}
