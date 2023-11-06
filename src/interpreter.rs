use crate::ast::{Expr, Visitor};
use crate::error::RuntimeError;
use crate::token::{Literal, TokenType};
use std::fmt;

#[derive(Debug)]
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

pub struct Interpretor;

impl Interpretor {
    pub fn interpret(&mut self, expression: &Expr) -> Result<Object, RuntimeError> {
        self.visit_expr(expression)
    }
}

impl Visitor<Result<Object, RuntimeError>> for Interpretor {
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match e {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(left, t, right) => match t.token_type {
                TokenType::BangEqual => Ok(Object::Bool(!is_equal(
                    &self.visit_expr(left)?,
                    &self.visit_expr(right)?,
                ))),
                TokenType::EqualEqual => Ok(Object::Bool(is_equal(
                    &self.visit_expr(left)?,
                    &self.visit_expr(right)?,
                ))),
                TokenType::Greater => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l > r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                TokenType::GreaterEqual => {
                    match (self.visit_expr(left)?, self.visit_expr(right)?) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l >= r)),
                        (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                    }
                }
                TokenType::Less => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l < r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                TokenType::LessEqual => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Bool(l <= r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                TokenType::Minus => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l - r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                TokenType::Plus => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                    (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                    (_, _) => Err(RuntimeError::new(
                        t.clone(),
                        "Operands must be two numbers or two strings.",
                    )),
                },
                TokenType::Slash => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l / r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                TokenType::Star => match (self.visit_expr(left)?, self.visit_expr(right)?) {
                    (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l * r)),
                    (_, _) => Err(RuntimeError::new(t.clone(), "Operands must be numbers.")),
                },
                _ => Ok(Object::Nil),
            },

            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Literal(literal) => match literal {
                Literal::String(val) => Ok(Object::String(val.to_string())),
                Literal::Number(val) => Ok(Object::Number(*val)),
                Literal::True => Ok(Object::Bool(true)),
                Literal::False => Ok(Object::Bool(false)),
                Literal::Nil => Ok(Object::Nil),
            },
            Expr::Unary(t, e) => match t.token_type {
                TokenType::Bang => Ok(Object::Bool(is_truthy(&self.visit_expr(e)?))),
                TokenType::Minus => match self.visit_expr(e)? {
                    Object::Number(n) => Ok(Object::Number(-n)),
                    _ => Err(RuntimeError::new(t.clone(), "Operand must be a number")),
                },
                _ => Ok(Object::Nil),
            },
        }
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
    use crate::token::{Token, TokenType};

    #[test]
    fn unary() {
        let mut interpretor = Interpretor;
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
}
