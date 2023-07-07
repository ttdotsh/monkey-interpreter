use crate::{
    ast::{Args, Ast, Block, Expr, Operator, Params, Stmt},
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
        Stmt::Let {
            ident: String::from("x").into(),
            val: Expr::IntLiteral(5),
        },
        Stmt::Let {
            ident: String::from("y").into(),
            val: Expr::IntLiteral(10),
        },
        Stmt::Let {
            ident: String::from("foobar").into(),
            val: Expr::IntLiteral(838383),
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
        Stmt::Return(Expr::IntLiteral(5)),
        Stmt::Return(Expr::IntLiteral(10)),
        Stmt::Return(Expr::IntLiteral(993322)),
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

    let expected_statement = Stmt::Expression(Expr::Ident(String::from("foobar")));
    assert_eq!(expected_statement, program[0]);
}

#[test]
fn test_parse_int_literal_expression() {
    let (program, errors) = test("5;");

    assert!(errors.is_empty());
    assert_eq!(program.len(), 1);

    let expected_statement = Stmt::Expression(Expr::IntLiteral(5));
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
        Stmt::Expression(Expr::BooleanLiteral(true)),
        Stmt::Expression(Expr::BooleanLiteral(false)),
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
        Stmt::Expression(Expr::Prefix(Operator::Bang, Box::new(Expr::IntLiteral(5)))),
        Stmt::Expression(Expr::Prefix(
            Operator::Minus,
            Box::new(Expr::IntLiteral(15)),
        )),
        Stmt::Expression(Expr::Prefix(
            Operator::Bang,
            Box::new(Expr::BooleanLiteral(true)),
        )),
        Stmt::Expression(Expr::Prefix(
            Operator::Bang,
            Box::new(Expr::BooleanLiteral(false)),
        )),
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
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::Plus,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::Minus,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::Multiplication,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::Division,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::GreaterThan,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::LessThan,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::Equals,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::IntLiteral(5)),
            Operator::NotEquals,
            Box::new(Expr::IntLiteral(5)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::BooleanLiteral(true)),
            Operator::Equals,
            Box::new(Expr::BooleanLiteral(true)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::BooleanLiteral(true)),
            Operator::NotEquals,
            Box::new(Expr::BooleanLiteral(false)),
        )),
        Stmt::Expression(Expr::Infix(
            Box::new(Expr::BooleanLiteral(false)),
            Operator::Equals,
            Box::new(Expr::BooleanLiteral(false)),
        )),
    ];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}

#[test]
fn test_operator_precedence_parsing() -> std::fmt::Result {
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
        Stmt::Expression(Expr::If {
            condition: Box::new(Expr::Infix(
                Box::new(Expr::Ident(String::from("x"))),
                Operator::LessThan,
                Box::new(Expr::Ident(String::from("y"))),
            )),
            consequence: Block::from(vec![Stmt::Expression(Expr::Ident(
                String::from("x").into(),
            ))]),
            alternative: None,
        }),
        Stmt::Expression(Expr::If {
            condition: Box::new(Expr::Infix(
                Box::new(Expr::Ident(String::from("x"))),
                Operator::LessThan,
                Box::new(Expr::Ident(String::from("y"))),
            )),
            consequence: Block::from(vec![Stmt::Expression(Expr::Ident(
                String::from("x").into(),
            ))]),
            alternative: Some(Block::from(vec![Stmt::Expression(Expr::Ident(
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
        Stmt::Expression(Expr::FuncLiteral {
            parameters: Params::from(vec![
                Expr::Ident(String::from("x").into()),
                Expr::Ident(String::from("y").into()),
            ]),
            body: Block::from(vec![Stmt::Expression(Expr::Infix(
                Box::new(Expr::Ident(String::from("x"))),
                Operator::Plus,
                Box::new(Expr::Ident(String::from("y"))),
            ))]),
        }),
        Stmt::Expression(Expr::FuncLiteral {
            parameters: Params::from(vec![]),
            body: Block::from(vec![Stmt::Expression(Expr::Infix(
                Box::new(Expr::Ident(String::from("x"))),
                Operator::Plus,
                Box::new(Expr::Ident(String::from("y"))),
            ))]),
        }),
        Stmt::Expression(Expr::FuncLiteral {
            parameters: Params::from(vec![Expr::Ident(String::from("x"))]),
            body: Block::from(vec![Stmt::Expression(Expr::Infix(
                Box::new(Expr::Ident(String::from("x"))),
                Operator::Plus,
                Box::new(Expr::Ident(String::from("y"))),
            ))]),
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

    let expected_statements = vec![Stmt::Expression(Expr::Call {
        func_name: Box::new(Expr::Ident(String::from("add").into())),
        arguments: Args::from(vec![
            Expr::IntLiteral(1),
            Expr::Infix(
                Box::new(Expr::IntLiteral(2)),
                Operator::Multiplication,
                Box::new(Expr::IntLiteral(3)),
            ),
            Expr::Infix(
                Box::new(Expr::IntLiteral(4)),
                Operator::Plus,
                Box::new(Expr::IntLiteral(5)),
            ),
        ]),
    })];

    assert_eq!(expected_statements.len(), program.len());

    expected_statements
        .into_iter()
        .enumerate()
        .for_each(|(i, s)| assert_eq!(s, program[i]));
}
