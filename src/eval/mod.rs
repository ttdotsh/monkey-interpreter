mod object;
#[cfg(test)]
mod test;

use crate::ast::{Ast, Expr, Operator, Stmt};
use object::Object;

type Result<T> = std::result::Result<T, String>;

/*
* Evaluation functions for different Node types
*/
pub fn eval_program(Ast(statements): Ast) -> Object {
    let mut obj = Object::Null;
    for s in statements {
        match eval_statement(s) {
            Ok(Object::ReturnValue(rv)) => return *rv,
            Err(s) => return Object::Error(s),
            Ok(o) => obj = o,
        }
    }
    obj
}

fn eval_block(Ast(statements): Ast) -> Result<Object> {
    let mut obj = Object::Null;
    for s in statements {
        match eval_statement(s)? {
            rv @ Object::ReturnValue(_) => return Ok(rv),
            o => obj = o,
        }
    }
    Ok(obj)
}

fn eval_statement(stmt: Stmt) -> Result<Object> {
    match stmt {
        Stmt::Let { .. } => todo!(),
        Stmt::Return(expr) => {
            let ret_val = eval_expression(expr)?;
            Ok(Object::ReturnValue(Box::new(ret_val)))
        }
        Stmt::Expression(expr) => eval_expression(expr),
    }
}

fn eval_expression(expr: Expr) -> Result<Object> {
    match expr {
        /* Literals */
        Expr::IntLiteral(i) => Ok(Object::Integer(i)),
        Expr::BooleanLiteral(b) => Ok(Object::Boolean(b)),

        /* Keyword-based Expressions */
        Expr::If { check, block, alt } => eval_if_expr(*check, block, alt),

        /* Operator-based Expressions */
        Expr::Prefix(op, right) => eval_prefix_expr(op, *right),
        Expr::Infix(left, op, right) => eval_infix_expr(*left, op, *right),

        _ => Err(format!("Unsupported expression type: {}", expr)),
    }
}

fn eval_if_expr(check: Expr, block: Ast, alt: Option<Ast>) -> Result<Object> {
    if eval_expression(check)?.is_truthy() {
        Ok(eval_block(block)?)
    } else if let Some(else_block) = alt {
        Ok(eval_block(else_block)?)
    } else {
        Ok(Object::Null)
    }
}

fn eval_prefix_expr(op: Operator, right: Expr) -> Result<Object> {
    let operand = eval_expression(right)?;
    match op {
        Operator::Bang => Ok(eval_bang_prefix(operand)),
        Operator::Minus => Ok(eval_minus_prefix(operand)?),
        _ => Err(format!("Unsupported operator as prefix: {}", op)),
    }
}

fn eval_infix_expr(left: Expr, op: Operator, right: Expr) -> Result<Object> {
    match op {
        Operator::Plus => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_plus_infix(left, right)
        }
        Operator::Minus => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_minus_infix(left, right)
        }
        Operator::Multiplication => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_mult_infix(left, right)
        }
        Operator::Division => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_div_infix(left, right)
        }
        Operator::LessThan => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_less_infix(left, right)
        }
        Operator::GreaterThan => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_greater_infix(left, right)
        }
        Operator::Equals => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_equal_infix(left, right)
        }
        Operator::NotEquals => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_not_equal_infix(left, right)
        }
        invalid_op => Err(format!("Unsupported operator as infix: {}", invalid_op)),
    }
}

/*
* Prefix Expressions
*/
fn eval_bang_prefix(value: Object) -> Object {
    Object::Boolean(!value.is_truthy())
}

fn eval_minus_prefix(value: Object) -> Result<Object> {
    match value {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Err(format!("No such negative value of {}", value)),
    }
}

/*
* Infix Expressions
*/
fn eval_plus_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
        (l, r) => Err(format!("Cannot add {} to {}", l, r)),
    }
}

fn eval_minus_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
        (l, r) => Err(format!("Cannot subtract {} from {}", l, r)),
    }
}

fn eval_mult_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
        (l, r) => Err(format!("Cannot multiply {} and {}", l, r)),
    }
}

fn eval_div_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
        (l, r) => Err(format!("Cannot divide {} and {}", l, r)),
    }
}

fn eval_less_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l < r)),
        (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
    }
}

fn eval_greater_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l > r)),
        (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
    }
}

fn eval_equal_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l == r)),
        (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l == r)),
        (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
    }
}

fn eval_not_equal_infix(left: Object, right: Object) -> Result<Object> {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l != r)),
        (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l != r)),
        (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
    }
}
