use crate::ast::{Expr, Visitor};
use crate::error::RuntimeError;
use crate::stmt::Stmt;
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
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> () {
        for stmt in stmts {
            <Interpretor as Visitor<()>>::visit_stmt(self, &stmt);
        }
    }
}

impl Visitor<Result<Object, RuntimeError>> for Interpretor {
    fn visit_expr(&mut self, e: &Expr) -> Result<Object, RuntimeError> {
        match e {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(left, t, right) => {
                let left_obj: Object =
                    <Interpretor as Visitor<Result<Object, RuntimeError>>>::visit_expr(self, left)?;
                let right_obj: Object =
                    <Interpretor as Visitor<Result<Object, RuntimeError>>>::visit_expr(
                        self, right,
                    )?;

                return match t.token_type {
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
                };
            }

            Expr::Grouping(e) => {
                <Interpretor as Visitor<Result<Object, RuntimeError>>>::visit_expr(self, e)
            }
            Expr::Literal(literal) => match literal {
                Literal::String(val) => Ok(Object::String(val.to_string())),
                Literal::Number(val) => Ok(Object::Number(*val)),
                Literal::True => Ok(Object::Bool(true)),
                Literal::False => Ok(Object::Bool(false)),
                Literal::Nil => Ok(Object::Nil),
            },
            Expr::Unary(t, e) => {
                let obj: Object =
                    <Interpretor as Visitor<Result<Object, RuntimeError>>>::visit_expr(self, e)?;
                return match t.token_type {
                    TokenType::Bang => Ok(Object::Bool(is_truthy(&obj))),
                    TokenType::Minus => match obj {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(RuntimeError::new(t.clone(), "Operand must be a number")),
                    },
                    _ => Ok(Object::Nil),
                };
            }
            Expr::Variable(_) => todo!(),
        }
    }
    fn visit_stmt(&mut self, e: &Stmt) -> Result<Object, RuntimeError> {
        todo!();
    }
}

impl Visitor<()> for Interpretor {
    fn visit_expr(&mut self, e: &Expr) -> () {}
    fn visit_stmt(&mut self, s: &Stmt) -> () {
        match s {
            Stmt::Expr(e) => self.visit_expr(e),
            Stmt::Print(e) => {
                let obj =
                    <Interpretor as Visitor<Result<Object, RuntimeError>>>::visit_expr(self, e);
                match obj {
                    Ok(o) => {
                        println!("{o}");
                    }
                    Err(e) => {
                        todo!();
                    }
                }
            }
            _ => (),
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
