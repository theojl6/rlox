use crate::ast::{Expr, Visitor};
use crate::token::{Literal, TokenType};

pub enum Object {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}

pub struct Interpretor;

impl Visitor<Object> for Interpretor {
    fn visit_expr(&mut self, e: &Expr) -> Object {
        match e {
            Expr::Assign(_, _) => todo!(),
            Expr::Binary(left, t, right) => match t.token_type {
                TokenType::BangEqual => {
                    Object::Bool(is_equal(&self.visit_expr(left), &self.visit_expr(right)))
                }
                TokenType::Greater => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Bool(l > r),
                    (_, _) => Object::Nil,
                },
                TokenType::GreaterEqual => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Bool(l >= r),
                    (_, _) => Object::Nil,
                },
                TokenType::Less => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Bool(l < r),
                    (_, _) => Object::Nil,
                },
                TokenType::LessEqual => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Bool(l <= r),
                    (_, _) => Object::Nil,
                },
                TokenType::Minus => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(l - r),
                    (_, _) => Object::Nil,
                },
                TokenType::Plus => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
                    (Object::String(l), Object::String(r)) => Object::String(l + &r),
                    (_, _) => Object::Nil,
                },
                TokenType::Slash => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(l / r),
                    (_, _) => Object::Nil,
                },
                TokenType::Star => match (self.visit_expr(left), self.visit_expr(right)) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(l * r),
                    (_, _) => Object::Nil,
                },
                _ => Object::Nil,
            },

            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Literal(literal) => match literal {
                Literal::String(val) => Object::String(val.to_string()),
                Literal::Number(val) => Object::Number(*val),
                Literal::True => Object::Bool(true),
                Literal::False => Object::Bool(false),
                Literal::Nil => Object::Nil,
            },
            Expr::Unary(t, e) => match t.token_type {
                TokenType::Minus => match self.visit_expr(e) {
                    Object::Number(n) => Object::Number(-n),
                    _ => Object::Nil,
                },
                TokenType::Bang => Object::Bool(is_truthy(&self.visit_expr(e))),
                _ => Object::Nil,
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
