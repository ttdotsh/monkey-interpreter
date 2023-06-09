use crate::{ast::Program, lexer::Lexer, token::Token};
use std::mem::swap;

#[allow(dead_code)]
struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

#[allow(dead_code)]
impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        return Parser {
            lexer,
            current_token,
            peek_token,
        };
    }

    fn step(&mut self) {
        swap(&mut self.current_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();
        return program;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{expression::Identifier, statement::Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_parse_let_statements() {
        let test_input = r#"
            let five = 5;
            let ten = 10;
            let foobar = 838383;
        "#;
        let lexer = Lexer::new(test_input.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected_indents = vec![
            Identifier(String::from("five")),
            Identifier(String::from("ten")),
            Identifier(String::from("foobar")),
        ];
        // TODO: add expected_expressions and test those here as well
        for (i, Statement::Let(ls)) in program.statements.into_iter().enumerate() {
            assert_eq!(ls.name, expected_indents[i]);
        }
    }
}
