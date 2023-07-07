mod object;

use self::object::Object;
use crate::ast::{Ast, Block, Expr, Operator, Stmt};

/*
* Evaluation functions for different Node types
*/
#[allow(unused)]
pub fn eval(ast: &Ast) -> Object {
    let mut obj = Object::Null;
    ast.iter().for_each(|s| obj = eval_statement(s));
    obj
}

fn eval_statement(stmt: &Stmt) -> Object {
    match stmt {
        Stmt::Let { .. } => todo!(),
        Stmt::Return(expr) => eval_expression(expr),
        Stmt::Expression(expr) => eval_expression(expr),
    }
}

fn eval_expression(expr: &Expr) -> Object {
    match expr {
        Expr::IntLiteral(i) => Object::Integer(*i),
        Expr::BooleanLiteral(b) => Object::Boolean(*b),
        Expr::Prefix(op, right) => eval_prefix_expression(op, right),
        Expr::Infix(left, op, right) => eval_infix_expression(left, op, right),
        Expr::If {
            condition,
            consequence,
            alternative,
        } => {
            let alt = if let Some(b) = alternative {
                Some(b)
            } else {
                None
            };
            eval_if_expr(condition, consequence, alt)
        }
        _ => todo!(),
    }
}

/*
* Prefix Expressions
*/
fn eval_prefix_expression(op: &Operator, right: &Expr) -> Object {
    let operand = eval_expression(right);
    match op {
        Operator::Bang => eval_bang_prefix(operand),
        Operator::Minus => eval_minus_prefix(operand),
        _ => todo!(),
    }
}

fn eval_bang_prefix(value: Object) -> Object {
    match value {
        Object::Boolean(b) => Object::Boolean(!b),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix(value: Object) -> Object {
    match value {
        Object::Integer(i) => Object::Integer(-i),
        _ => Object::Null,
    }
}

/*
* Infix Expressions
*/
fn eval_infix_expression(left: &Expr, op: &Operator, right: &Expr) -> Object {
    match op {
        Operator::Plus => eval_plus_infix(eval_expression(left), eval_expression(right)),
        Operator::Minus => eval_minus_infix(eval_expression(left), eval_expression(right)),
        Operator::Multiplication => eval_mult_infix(eval_expression(left), eval_expression(right)),
        Operator::Division => eval_div_infix(eval_expression(left), eval_expression(right)),
        Operator::LessThan => eval_less_infix(eval_expression(left), eval_expression(right)),
        Operator::GreaterThan => eval_greater_infix(eval_expression(left), eval_expression(right)),
        Operator::Equals => eval_equal_infix(eval_expression(left), eval_expression(right)),
        Operator::NotEquals => eval_not_equal_infix(eval_expression(left), eval_expression(right)),
        _ => Object::Null,
    }
}

fn eval_plus_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Integer(l + r),
        _ => Object::Null,
    }
}

fn eval_minus_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Integer(l - r),
        _ => Object::Null,
    }
}

fn eval_mult_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Integer(l * r),
        _ => Object::Null,
    }
}

fn eval_div_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Integer(l / r),
        _ => Object::Null,
    }
}

fn eval_less_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Boolean(l < r),
        _ => Object::Null,
    }
}

fn eval_greater_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Boolean(l > r),
        _ => Object::Null,
    }
}

fn eval_equal_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Boolean(l == r),
        (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l == r),
        _ => Object::Null,
    }
}

fn eval_not_equal_infix(left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Object::Boolean(l != r),
        (Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l != r),
        _ => Object::Null,
    }
}

/*
* If Expressions
*/
fn eval_if_expr(cond: &Expr, consequence: &Block, alt: Option<&Block>) -> Object {
    if eval_expression(cond).is_truthy() {
        eval(consequence)
    } else {
        if let Some(b) = alt {
            eval(b)
        } else {
            Object::Null
        }
    }
}

#[cfg(test)]
mod test {
    use super::{eval, object::Object};
    use crate::parse::Parser;

    fn test(src: &str) -> Object {
        let mut parser = Parser::new(src);
        let program = parser.parse();
        eval(&program)
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
}
