use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::Object;
use crate::token::{Token, TokenType};

pub trait LoxError {
    fn report(&self);
}

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
    pub value: Option<Rc<RefCell<Object>>>,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str, value: Option<Rc<RefCell<Object>>>) -> Self {
        Self {
            token,
            message: message.into(),
            value,
        }
    }
}

impl LoxError for RuntimeError {
    fn report(&self) {
        if self.value.is_none() {
            println!(
                "[line {}] Error {}: {}",
                self.token.line, self.token.lexeme, self.message
            );
        }
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    token: Token,
    message: String,
}

impl SyntaxError {
    pub fn new(token: Token, message: &str) -> Self {
        report(token.line, &token.lexeme, &message);

        return Self {
            token,
            message: message.into(),
        };
    }
}

pub fn error(line: usize, message: &str) {
    report(line, &"", message);
}

pub fn report(line: usize, at: &str, message: &str) {
    println!("[line {line}] Error {at}: {message}");
}

pub fn lox_error(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, "at end", message);
    } else {
        let at = format!(" at {}'", &token.lexeme);
        report(token.line, &at, message);
    }
}
