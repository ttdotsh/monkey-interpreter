use crate::{
    ast::{Arguments, Ast, Block, Expression, Operator, Parameters, Statement},
    parse::{ParseError, Parser},
    token::Token,
};

fn test(src: &str) -> (Ast, Vec<ParseError>) {
    let mut parser = Parser::new(src);
    (parser.parse(), parser.errors)
}

#[test]
fn test_parse_let_statements() {
    let (program, errors) = test(
        r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![
        Statement::Let {
            name: String::from("x").into(),
            value: Expression::IntLiteral(5),
        },
        Statement::Let {
            name: String::from("y").into(),
            value: Expression::IntLiteral(10),
        },
        Statement::Let {
            name: String::from("foobar").into(),
            value: Expression::IntLiteral(838383),
        },
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_parse_return_statement() {
    let (program, errors) = test(
        r#"
            return 5;
            return 10;
            return 993322;
        "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![
        Statement::Return(Expression::IntLiteral(5)),
        Statement::Return(Expression::IntLiteral(10)),
        Statement::Return(Expression::IntLiteral(993322)),
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_let_statement_syntax_errors() {
    let (_, errors) = test(
        r#"
            let = 5;
            let y y 10;
        "#,
    );

    let expected_errors = vec![
        ParseError::ExpectedIdentifier,
        ParseError::UnexpectedToken {
            expected: Token::Assign,
            recieved: Token::Ident(String::from("y")),
        },
    ];

    expected_errors
        .into_iter()
        .for_each(|e| assert!(errors.contains(&e)));
}

#[test]
fn test_parse_identifier_expression() {
    let (program, errors) = test("foobar;");

    assert!(errors.is_empty());
    assert_eq!(program.len(), 1);

    let expected_statement =
        Statement::Expression(Expression::Ident(String::from("foobar").into()));
    assert_eq!(expected_statement, program[0]);
}

#[test]
fn test_parse_int_literal_expression() {
    let (program, errors) = test("5;");

    assert!(errors.is_empty());
    assert_eq!(program.len(), 1);

    let expected_statement = Statement::Expression(Expression::IntLiteral(5));
    assert_eq!(expected_statement, program[0]);
}

#[test]
fn test_parse_boolean_literal_expression() {
    let (program, errors) = test(
        r#"
            true;
            false;
        "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![
        Statement::Expression(Expression::BooleanLiteral(true)),
        Statement::Expression(Expression::BooleanLiteral(false)),
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_parse_prefix_expression() {
    let (program, errors) = test(
        r#"
            !5;
            -15;
            !true;
            !false;
        "#,
    );

    assert!(errors.is_empty());

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

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_parse_infix_expression() {
    let (program, errors) = test(
        r#"
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
       "#,
    );

    assert!(errors.is_empty());

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

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
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
        let (program, _errors) = test(expr);
        let ast_string = program
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("");

        assert_eq!(ast_string, expect);
    }
    Ok(())
}

#[test]
fn test_if_expression() {
    let (program, errors) = test(
        r#"
            if (x < y) { x }
            if (x < y) { x } else { y }
        "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![
        Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x").into())),
                operator: Operator::LessThan,
                right: Box::new(Expression::Ident(String::from("y").into())),
            }),
            consequence: Block(vec![Statement::Expression(Expression::Ident(
                String::from("x").into(),
            ))]),
            alternative: None,
        }),
        Statement::Expression(Expression::If {
            condition: Box::new(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x").into())),
                operator: Operator::LessThan,
                right: Box::new(Expression::Ident(String::from("y").into())),
            }),
            consequence: Block(vec![Statement::Expression(Expression::Ident(
                String::from("x").into(),
            ))]),
            alternative: Some(Block(vec![Statement::Expression(Expression::Ident(
                String::from("y").into(),
            ))])),
        }),
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_parse_function_literal() {
    let (program, errors) = test(
        r#"
            fn(x, y) { x + y; }
            fn() { x + y; }
            fn(x) { x + y; }
        "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![
                Expression::Ident(String::from("x").into()),
                Expression::Ident(String::from("y").into()),
            ]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x").into())),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y").into())),
            })]),
        }),
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x").into())),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y").into())),
            })]),
        }),
        Statement::Expression(Expression::FuncLiteral {
            parameters: Parameters(vec![Expression::Ident(String::from("x").into())]),
            body: Block(vec![Statement::Expression(Expression::Infix {
                left: Box::new(Expression::Ident(String::from("x").into())),
                operator: Operator::Plus,
                right: Box::new(Expression::Ident(String::from("y").into())),
            })]),
        }),
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_parse_call_expression() {
    let (program, errors) = test(
        r#"
        add(1, 2 * 3, 4 + 5);
    "#,
    );

    assert!(errors.is_empty());

    let expected_statements = vec![Statement::Expression(Expression::Call {
        function: Box::new(Expression::Ident(String::from("add").into())),
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

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}
