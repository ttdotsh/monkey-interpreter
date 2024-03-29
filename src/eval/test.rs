use super::{
    super::ast::{Ast, Params},
    object::Object,
    Runtime,
};
use crate::{
    ast::{Expr, Operator, Stmt},
    parse::Parser,
};

fn test(src: &str) -> Object {
    let mut parser = Parser::new(src);
    let program = parser.parse();
    let env = Runtime::new();
    env.evaluate(program)
}

#[test]
fn test_eval_int_expression() {
    let input_and_expected = vec![
        ("5", Object::Integer(5)),
        ("10", Object::Integer(10)),
        ("42069", Object::Integer(42069)),
        ("-5", Object::Integer(-5)),
        ("-10", Object::Integer(-10)),
        ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
        ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
        ("-50 + 100 + -50", Object::Integer(0)),
        ("5 * 2 + 10", Object::Integer(20)),
        ("5 + 2 * 10", Object::Integer(25)),
        ("20 + 2 * -10", Object::Integer(0)),
        ("50 / 2 * 2 + 10", Object::Integer(60)),
        ("2 * (5 + 10)", Object::Integer(30)),
        ("3 * 3 * 3 + 10", Object::Integer(37)),
        ("3 * (3 * 3) + 10", Object::Integer(37)),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_bool_expression() {
    let input_and_expected = vec![
        ("true", Object::Boolean(true)),
        ("false", Object::Boolean(false)),
        ("1 < 2", Object::Boolean(true)),
        ("1 > 2", Object::Boolean(false)),
        ("1 < 1", Object::Boolean(false)),
        ("1 > 1", Object::Boolean(false)),
        ("1 == 1", Object::Boolean(true)),
        ("1 != 1", Object::Boolean(false)),
        ("1 == 2", Object::Boolean(false)),
        ("1 != 2", Object::Boolean(true)),
        ("true == true", Object::Boolean(true)),
        ("false == false", Object::Boolean(true)),
        ("true == false", Object::Boolean(false)),
        ("true != false", Object::Boolean(true)),
        ("false != true", Object::Boolean(true)),
        ("(1 < 2) == true", Object::Boolean(true)),
        ("(1 < 2) == false", Object::Boolean(false)),
        ("(1 > 2) == true", Object::Boolean(false)),
        ("(1 > 2) == false", Object::Boolean(true)),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_prefix_expression() {
    let input_and_expected = vec![
        ("!true", Object::Boolean(false)),
        ("!false", Object::Boolean(true)),
        ("!!false", Object::Boolean(false)),
        ("!!true", Object::Boolean(true)),
        ("!5", Object::Boolean(false)),
        ("!!5", Object::Boolean(true)),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_if_expression() {
    let input_and_expected = vec![
        ("if (true) { 10 }", Object::Integer(10)),
        ("if (false) { 10 }", Object::Null),
        ("if (1) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 }", Object::Integer(10)),
        ("if (1 > 2) { 10 }", Object::Null),
        ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_return_stmt() {
    let input_and_expected = vec![
        ("return 10;", Object::Integer(10)),
        ("return 10; 9;", Object::Integer(10)),
        ("return 2 * 5; 9;", Object::Integer(10)),
        ("9; return 2 * 5; 9;", Object::Integer(10)),
        (
            r#"
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1; 
                }
                "#,
            Object::Integer(10),
        ),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_errors() {
    let input_and_expected = vec![
        ("5 + true;", Object::Error("Cannot add 5 to true".into())),
        ("5 + true; 5;", Object::Error("Cannot add 5 to true".into())),
        (
            "-true",
            Object::Error("No such negative value of true".into()),
        ),
        (
            "true + false;",
            Object::Error("Cannot add true to false".into()),
        ),
        (
            "5; true + false; 5",
            Object::Error("Cannot add true to false".into()),
        ),
        (
            "if (10 > 1) { true + false; }",
            Object::Error("Cannot add true to false".into()),
        ),
        (
            r#" 
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1; 
                }
                "#,
            Object::Error("Cannot add true to false".into()),
        ),
        (
            "foobar",
            Object::Error("Identifier not found: foobar".into()),
        ),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_let_stmts() {
    let input_and_expected = vec![
        ("let a = 5; a;", Object::Integer(5)),
        ("let a = 5 * 5; a;", Object::Integer(25)),
        ("let a = 5; let b = a; b;", Object::Integer(5)),
        (
            "let a = 5; let b = a; let c = a + b + 5; c;",
            Object::Integer(15),
        ),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}

#[test]
fn test_eval_func_def() {
    let input = "fn(x) { x + 2; };";

    /*
     * Here, we deviate from the typical pattern of these tests because the Environments
     * will not be equivalent. An Rc created in this test will not point to the same underlying
     * Environment allocation in the `test` function, and so the two Object::Func's won't be equal
     */
    let expected_params = Params::from(vec![Expr::Ident("x".into())]);
    let expected_body = Ast::from(vec![Stmt::Expression(Expr::Infix(
        Box::new(Expr::Ident("x".into())),
        Operator::Plus,
        Box::new(Expr::IntLiteral(2)),
    ))]);

    let obj = test(input);
    match obj {
        Object::Func { params, body, .. } => {
            assert_eq!(params, expected_params);
            assert_eq!(body, expected_body);
        }
        _ => assert!(false),
    }
}

#[test]
fn test_eval_func_call() {
    let input_and_expected = vec![
        (
            "let identity = fn(x) { x; }; identity(5);",
            Object::Integer(5),
        ),
        (
            "let identity = fn(x) { return x; }; identity(5);",
            Object::Integer(5),
        ),
        (
            "let double = fn(x) { x * 2; }; double(5);",
            Object::Integer(10),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5, 5);",
            Object::Integer(10),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            Object::Integer(20),
        ),
        ("fn(x) { x; }(5)", Object::Integer(5)),
    ];
    input_and_expected
        .into_iter()
        .for_each(|(i, e)| assert_eq!(test(i), e))
}
