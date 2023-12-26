use crate::ast::Expr;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::stmt::Stmt;
use crate::token::TokenType;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Bool(b) => {
                write!(f, "{:}", b)
            }
            Object::String(s) => {
                write!(f, "{:}", s)
            }
            Object::Number(n) => {
                write!(f, "{:}", n)
            }
            Object::Nil => {
                write!(f, "{:}", "nil")
            }
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        match (self, other) {
            (&Object::Number(l), &Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            (Object::Bool(l), Object::Bool(r)) => l == r,
            (Object::Nil, Object::Nil) => true,
            (_, _) => false,
        }
    }
}

pub struct Interpretor {
    environment: Rc<RefCell<Environment>>,
}

impl Interpretor {
    pub fn new() -> Self {
        Interpretor {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> () {
        for stmt in stmts {
            let _ = self.visit_stmt(&stmt);
        }
    }
    fn interpret_block(&mut self, stmts: &Vec<Stmt>, environment: Rc<RefCell<Environment>>) {
        let previous = self.environment.clone();
        self.environment = environment;
        for stmt in stmts {
            let _ = self.visit_stmt(&stmt);
        }
        self.environment = previous;
    }
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match e {
            Expr::Assign(t, e) => {
                let value = self.visit_expr(e)?;
                let v = value.clone();
                self.environment.borrow_mut().assign(t.clone(), v)?;
                return Ok(value);
            }
            Expr::Binary(left, t, right) => {
                let left_obj = self.visit_expr(left)?;
                let right_obj = self.visit_expr(right)?;

                match t.token_type {
                    TokenType::BangEqual => Ok(Object::Bool(!is_equal(&left_obj, &right_obj))),
                    TokenType::EqualEqual => Ok(Object::Bool(is_equal(&left_obj, &right_obj))),
                    TokenType::Greater => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l > r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::GreaterEqual => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l >= r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::Less => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l < r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::LessEqual => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l <= r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::Minus => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::Plus => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                        (_, _) => Err(RuntimeError::new(
                            t.clone(),
                            "Operands must be two numbers or two strings.",
                        )),
                    },
                    TokenType::Slash => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    TokenType::Star => match (left_obj, right_obj) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    },
                    _ => Ok(Object::Nil),
                }
            }

            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Literal(o) => Ok(o.clone()),
            Expr::Logical(left, operator, right) => {
                let left = self.visit_expr(left)?;

                if operator.token_type == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&left) {
                        return Ok(left);
                    }
                }
                self.visit_expr(right)
            }

            Expr::Unary(t, e) => {
                let obj: Object = self.visit_expr(e)?;
                match t.token_type {
                    TokenType::Bang => Ok(Object::Bool(is_truthy(&obj))),
                    TokenType::Minus => match obj {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(RuntimeError::new(t.clone(), "Operand must be a number")),
                    },
                    _ => Ok(Object::Nil),
                }
            }
            Expr::Variable(t) => {
                let value = self.environment.borrow().get(t.clone())?;
                Ok(value.clone())
            }
        }
    }
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Expr(e) => {
                let _ = self.visit_expr(e)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.visit_expr(condition)?) {
                    self.visit_stmt(&then_branch)?;
                } else {
                    match else_branch {
                        Some(s) => {
                            self.visit_stmt(s)?;
                        }
                        None => {}
                    }
                }
            }
            Stmt::Print(e) => {
                let obj = self.visit_expr(e)?;
                println!("{obj}");
            }
            Stmt::Var { name, initializer } => {
                let mut value = Object::Nil;
                match initializer {
                    Some(i) => {
                        value = self.visit_expr(i)?;
                    }
                    None => {}
                }
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
            }
            Stmt::Block { statements } => {
                self.interpret_block(
                    statements,
                    Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                        &(self.environment),
                    ))))),
                );
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.visit_expr(condition)?) {
                    self.visit_stmt(body)?;
                }
            }

            _ => {}
        };
        Ok(())
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Bool(b) => *b,
        _ => true,
    }
}

fn is_equal(l_obj: &Object, r_obj: &Object) -> bool {
    match (l_obj, r_obj) {
        (Object::Number(l), Object::Number(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        (Object::Bool(l), Object::Bool(r)) => l == r,
        (Object::Nil, Object::Nil) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{self, Token, TokenType};

    #[test]
    fn unary() {
        let mut interpretor = Interpretor::new();
        let unary_expression = Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Object::Number(1.0))),
        );
        match interpretor.visit_expr(&unary_expression) {
            Ok(r) => assert_eq!(r, Object::Number(-1.0)),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn assignment() {
        let mut interpretor = Interpretor::new();
        let assignment_expression = Expr::Assign(
            Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("a"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Object::Number(1.0))),
        );
    }
}
