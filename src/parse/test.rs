use crate::{
    ast::{Arguments, Block, Expression, Operator, Parameters, Statement},
    lex::Lexer,
    parse::{ParseError, Parser},
    token::Token,
};

#[test]
fn test_parse_let_statements() {
    let test_input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);
    assert_eq!(program.statements.len(), 3);

    let expected_indents = vec![String::from("x"), String::from("y"), String::from("foobar")];

    for (i, statement) in program.statements.into_iter().enumerate() {
        match statement {
            Statement::Let { name, .. } => {
                assert_eq!(expected_indents[i], name)
                // TODO: add expected expressions here
            }
            _ => assert!(false),
        };
    }
}

#[test]
fn test_parse_return_statement() {
    let test_input = r#"
            return 5;
            return 10;
            return 993322;
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);
    assert_eq!(program.statements.len(), 3);

    for statement in program.statements {
        match statement {
            Statement::Return(_) => {}
            _ => assert!(false),
        }
    }
}

#[test]
fn test_let_statement_syntax_errors() {
    let test_input = r#"
            let = 5;
            let y y 10;
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let _program = parser.parse_program();

    let expected_errors = vec![
        ParseError::ExpectedIdentifier,
        ParseError::UnexpectedToken {
            expected: Token::Assign,
            recieved: Token::Ident(String::from("y")),
        },
    ];

    for e in expected_errors {
        assert!(parser.errors.contains(&e));
    }
}

#[test]
fn test_parse_identifier_expression() {
    let test_input = "foobar;";
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);
    assert_eq!(program.statements.len(), 1);

    let expected_statement = Statement::Expression(Expression::Ident(String::from("foobar")));
    assert_eq!(expected_statement, program.statements[0]);
}

#[test]
fn test_parse_int_literal_expression() {
    let test_input = "5;";
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);
    assert_eq!(program.statements.len(), 1);

    let expected_statement = Statement::Expression(Expression::IntLiteral(5));
    assert_eq!(expected_statement, program.statements[0]);
}

#[test]
fn test_parse_boolean_literal_expression() {
    let test_input = r#"
            true;
            false;
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![
        Statement::Expression(Expression::BooleanLiteral(true)),
        Statement::Expression(Expression::BooleanLiteral(false)),
    ];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}

#[test]
fn test_parse_prefix_expression() {
    let test_input = r#"
            !5;
            -15;
            !true;
            !false;
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![
        Statement::Expression(Expression::Prefix {
            operator: Operator::Bang,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Prefix {
            operator: Operator::Minus,
            right: Box::new(Expression::IntLiteral(15)),
        }),
        Statement::Expression(Expression::Prefix {
            operator: Operator::Bang,
            right: Box::new(Expression::BooleanLiteral(true)),
        }),
        Statement::Expression(Expression::Prefix {
            operator: Operator::Bang,
            right: Box::new(Expression::BooleanLiteral(false)),
        }),
    ];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}

#[test]
fn test_parse_infix_expression() {
    let test_input = r#"
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;
            true == true;
            true != false;
            false == false;
       "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::Plus,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::Minus,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::Multiplication,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::Division,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::GreaterThan,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::LessThan,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::Equals,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::IntLiteral(5)),
            operator: Operator::NotEquals,
            right: Box::new(Expression::IntLiteral(5)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::BooleanLiteral(true)),
            operator: Operator::Equals,
            right: Box::new(Expression::BooleanLiteral(true)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::BooleanLiteral(true)),
            operator: Operator::NotEquals,
            right: Box::new(Expression::BooleanLiteral(false)),
        }),
        Statement::Expression(Expression::Infix {
            left: Box::new(Expression::BooleanLiteral(false)),
            operator: Operator::Equals,
            right: Box::new(Expression::BooleanLiteral(false)),
        }),
    ];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}

#[test]
fn test_operator_precedence_parsing() -> std::fmt::Result {
    // These were copied from the book
    let expressions_and_expectations = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];

    for (expr, expect) in expressions_and_expectations {
        let lexer = Lexer::new(expr.into());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let ast_string = program
            .statements
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("");

        assert_eq!(ast_string, expect);
    }
    Ok(())
}

#[test]
fn test_if_expression() {
    let test_input = r#"
            if (x < y) { x }
            if (x < y) { x } else { y }
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![
        Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x"))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Ident(String::from("y"))),
            }),
            consequence: Block(vec![Statement::Expression(Expression::Ident(
                String::from("x"),
            ))]),
            alternative: None,
        }),
        Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x"))),
                operator: Operator::LessThan,
                right: Box::new(Expression::Ident(String::from("y"))),
            }),
            consequence: Block(vec![Statement::Expression(Expression::Ident(
                String::from("x"),
            ))]),
            alternative: Some(Block(vec![Statement::Expression(Expression::Ident(
                String::from("y"),
            ))])),
        }),
    ];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}

#[test]
fn test_parse_function_literal() {
    let test_input = r#"
            fn(x, y) { x + y; }
            fn() { x + y; }
            fn(x) { x + y; }
        "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![
                Expression::Ident(String::from("x")),
                Expression::Ident(String::from("y")),
            ]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x"))),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y"))),
            })]),
        }),
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x"))),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y"))),
            })]),
        }),
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![Expression::Ident(String::from("x"))]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x"))),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y"))),
            })]),
        }),
    ];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}

#[test]
fn test_parse_call_expression() {
    let test_input = r#"
        add(1, 2 * 3, 4 + 5);
    "#;
    let lexer = Lexer::new(test_input.into());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("{:?}", parser.errors);
    assert_eq!(parser.errors.len(), 0);

    let expected_statements = vec![Statement::Expression(Expression::Call {
        function: Box::new(Expression::Ident(String::from("add"))),
        arguments: Arguments(vec![
            Expression::IntLiteral(1),
            Expression::Infix {
                left: Box::new(Expression::IntLiteral(2)),
                operator: Operator::Multiplication,
                right: Box::new(Expression::IntLiteral(3)),
            },
            Expression::Infix {
                left: Box::new(Expression::IntLiteral(4)),
                operator: Operator::Plus,
                right: Box::new(Expression::IntLiteral(5)),
            },
        ]),
    })];

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program.statements[i]));
}
