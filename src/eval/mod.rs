mod env;
mod object;

use super::ast::{Ast, Expr, Operator, Stmt};
use env::Environment;
use object::Object;
use std::{cell::RefCell, rc::Rc};

pub struct Runtime<'e> {
    env: Rc<RefCell<Environment<'e>>>,
}

impl<'e> Runtime<'e> {
    pub fn new() -> Runtime<'e> {
        Runtime {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn evaluate(&self, ast: Ast) -> Object {
        match self.eval_ast(ast) {
            Ok(Object::ReturnValue(v)) => *v,
            Ok(o) => o,
            Err(s) => Object::Error(s),
        }
    }

    fn eval_ast(&self, Ast(statements): Ast) -> Result<Object, String> {
        let mut obj = Object::Null;

        for s in statements {
            match self.eval_statement(s)? {
                rv @ Object::ReturnValue(_) => return Ok(rv),
                o => obj = o,
            }
        }

        Ok(obj)
    }

    fn eval_statement(&self, stmt: Stmt) -> Result<Object, String> {
        match stmt {
            Stmt::Let { ident, val } => {
                let val = self.eval_expression(val)?;
                self.env.borrow_mut().set(ident, val.clone());
                Ok(val)
            }

            Stmt::Return(expr) => {
                let val = self.eval_expression(expr)?;
                Ok(Object::ReturnValue(Box::new(val)))
            }

            Stmt::Expression(expr) => self.eval_expression(expr),
        }
    }

    fn eval_expression(&self, expr: Expr) -> Result<Object, String> {
        match expr {
            Expr::IntLiteral(i) => Ok(Object::Integer(i)),
            Expr::BooleanLiteral(b) => Ok(Object::Boolean(b)),

            Expr::Ident(s) => match self.env.borrow().get(&s) {
                Some(obj) => Ok(obj),
                None => Err(format!("Identifier not found: {}", &s)),
            },

            Expr::If { check, block, alt } => {
                if self.eval_expression(*check)?.is_truthy() {
                    self.eval_ast(block)
                } else {
                    match alt {
                        Some(block) => self.eval_ast(block),
                        None => Ok(Object::Null),
                    }
                }
            }

            Expr::Prefix(op, right) => {
                let operand = self.eval_expression(*right)?;
                match op {
                    Operator::Bang => Ok(!operand),
                    Operator::Minus => -operand,
                    _ => Err(format!("Unsupported operator as prefix: {}", op)),
                }
            }

            Expr::Infix(left, op, right) => match op {
                Operator::Plus => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    left + right
                }
                Operator::Minus => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    left - right
                }
                Operator::Multiplication => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    left * right
                }
                Operator::Division => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    left / right
                }

                Operator::LessThan => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    Ok(Object::Boolean(left < right))
                }
                Operator::GreaterThan => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    Ok(Object::Boolean(left > right))
                }
                Operator::Equals => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    Ok(Object::Boolean(left == right))
                }
                Operator::NotEquals => {
                    let left = self.eval_expression(*left)?;
                    let right = self.eval_expression(*right)?;
                    Ok(Object::Boolean(left != right))
                }
                invalid_op => Err(format!("Unsupported operator as infix: {}", invalid_op)),
            },

            _ => Err(format!("Unsupported expression type: {}", expr)),
        }
    }
}

#[cfg(test)]
mod test;
