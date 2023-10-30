use crate::ast::{Expr, Visitor};
use crate::token::{Literal, TokenType};

pub enum Object {
    Expr(Box<Expr>),
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
            Expr::Binary(left, _, right) => todo!(),
            Expr::Grouping(e) => Object::Expr(e.clone()),
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
