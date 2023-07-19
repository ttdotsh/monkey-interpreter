mod object;
#[cfg(test)]
mod test;

use crate::ast::{Ast, Expr, Operator, Stmt};
use object::Object;
use std::{cell::RefCell, collections::HashMap};

type Result<T> = std::result::Result<T, String>;

pub struct Evaluator {
    env: RefCell<HashMap<String, Object>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: RefCell::new(HashMap::new()),
        }
    }

    pub fn eval(&self, Ast(statements): Ast) -> Object {
        let mut obj = Object::Null;
        for s in statements {
            match self.eval_statement(s) {
                Ok(Object::ReturnValue(rv)) => return *rv,
                Err(s) => return Object::Error(s),
                Ok(o) => obj = o,
            }
        }
        obj
    }

    fn eval_block(&self, Ast(statements): Ast) -> Result<Object> {
        let mut obj = Object::Null;
        for s in statements {
            match self.eval_statement(s)? {
                rv @ Object::ReturnValue(_) => return Ok(rv),
                o => obj = o,
            }
        }
        Ok(obj)
    }

    fn eval_statement(&self, stmt: Stmt) -> Result<Object> {
        match stmt {
            Stmt::Let { ident, val } => {
                let val = self.eval_expression(val)?;
                self.env.borrow_mut().insert(ident, val.clone());
                Ok(val)
            }
            Stmt::Return(expr) => {
                let ret_val = self.eval_expression(expr)?;
                Ok(Object::ReturnValue(Box::new(ret_val)))
            }
            Stmt::Expression(expr) => self.eval_expression(expr),
        }
    }

    fn eval_expression(&self, expr: Expr) -> Result<Object> {
        match expr {
            Expr::IntLiteral(i) => Ok(Object::Integer(i)),
            Expr::BooleanLiteral(b) => Ok(Object::Boolean(b)),

            Expr::Ident(s) => self.eval_ident(s),
            Expr::If { check, block, alt } => self.eval_if_expr(*check, block, alt),

            Expr::Prefix(op, right) => self.eval_prefix_expr(op, *right),
            Expr::Infix(left, op, right) => self.eval_infix_expr(*left, op, *right),

            _ => Err(format!("Unsupported expression type: {}", expr)),
        }
    }

    fn eval_ident(&self, name: String) -> Result<Object> {
        let env = self.env.borrow();
        let var = env.get(&name);
        match var {
            Some(o) => Ok(o.clone()),
            None => Err(format!("Identifier not found: {}", &name)),
        }
    }

    fn eval_if_expr(&self, check: Expr, block: Ast, alt: Option<Ast>) -> Result<Object> {
        if self.eval_expression(check)?.is_truthy() {
            Ok(self.eval_block(block)?)
        } else if let Some(else_block) = alt {
            Ok(self.eval_block(else_block)?)
        } else {
            Ok(Object::Null)
        }
    }

    fn eval_prefix_expr(&self, op: Operator, right: Expr) -> Result<Object> {
        let operand = self.eval_expression(right)?;
        match op {
            Operator::Bang => Ok(self.eval_bang_prefix(operand)),
            Operator::Minus => Ok(self.eval_minus_prefix(operand)?),
            _ => Err(format!("Unsupported operator as prefix: {}", op)),
        }
    }

    fn eval_infix_expr(&self, left: Expr, op: Operator, right: Expr) -> Result<Object> {
        match op {
            Operator::Plus => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_plus_infix(left, right)
            }
            Operator::Minus => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_minus_infix(left, right)
            }
            Operator::Multiplication => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_mult_infix(left, right)
            }
            Operator::Division => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_div_infix(left, right)
            }
            Operator::LessThan => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_less_infix(left, right)
            }
            Operator::GreaterThan => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_greater_infix(left, right)
            }
            Operator::Equals => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_equal_infix(left, right)
            }
            Operator::NotEquals => {
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;
                self.eval_not_equal_infix(left, right)
            }
            invalid_op => Err(format!("Unsupported operator as infix: {}", invalid_op)),
        }
    }

    /*
     * Prefix Expressions
     */
    fn eval_bang_prefix(&self, value: Object) -> Object {
        Object::Boolean(!value.is_truthy())
    }

    fn eval_minus_prefix(&self, value: Object) -> Result<Object> {
        match value {
            Object::Integer(i) => Ok(Object::Integer(-i)),
            _ => Err(format!("No such negative value of {}", value)),
        }
    }

    /*
     * Infix Expressions
     */
    fn eval_plus_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
            (l, r) => Err(format!("Cannot add {} to {}", l, r)),
        }
    }

    fn eval_minus_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l - r)),
            (l, r) => Err(format!("Cannot subtract {} from {}", l, r)),
        }
    }

    fn eval_mult_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
            (l, r) => Err(format!("Cannot multiply {} and {}", l, r)),
        }
    }

    fn eval_div_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l / r)),
            (l, r) => Err(format!("Cannot divide {} and {}", l, r)),
        }
    }

    fn eval_less_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l < r)),
            (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
        }
    }

    fn eval_greater_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l > r)),
            (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
        }
    }

    fn eval_equal_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l == r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l == r)),
            (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
        }
    }

    fn eval_not_equal_infix(&self, left: Object, right: Object) -> Result<Object> {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => Ok(Object::Boolean(l != r)),
            (Object::Boolean(l), Object::Boolean(r)) => Ok(Object::Boolean(l != r)),
            (l, r) => Err(format!("Cannot compare equality between {} and {}", l, r)),
        }
    }
}
