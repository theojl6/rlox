use crate::ast::Expr;
use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::stmt::Stmt;
use crate::token::{Literal, TokenType};
use std::collections::HashMap;
use std::fmt;

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
    environment: Environment,
}

impl Interpretor {
    pub fn new() -> Self {
        Interpretor {
            environment: Environment {
                values: HashMap::new(),
            },
        }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> () {
        for stmt in stmts {
            let _ = self.visit_stmt(&stmt);
        }
    }
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match e {
            Expr::Assign(t, e) => {
                let value = self.visit_expr(e)?;
                let v = value.clone();
                self.environment.assign(t.clone(), v)?;
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
            Expr::Literal(literal) => match literal {
                Literal::String(val) => Ok(Object::String(val.to_string())),
                Literal::Number(val) => Ok(Object::Number(*val)),
                Literal::True => Ok(Object::Bool(true)),
                Literal::False => Ok(Object::Bool(false)),
                Literal::Nil => Ok(Object::Nil),
            },
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
                let value = self.environment.get(t.clone())?;
                Ok(value.clone())
            }
        }
    }
    fn visit_stmt(&mut self, s: &Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Expr(e) => {
                let _ = self.visit_expr(e)?;
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
                self.environment.define(name.lexeme.clone(), value);
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
            Box::new(Expr::Literal(Literal::Number(1.0))),
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
            Box::new(Expr::Literal(Literal::Number(1.0))),
        );
    }
}
