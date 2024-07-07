use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::Object;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
    pub value: Option<Rc<RefCell<Object>>>,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str, value: Option<Rc<RefCell<Object>>>) -> Self {
        if value.is_none() {
            report(token.line, &token.lexeme, &message);
        }

        Self {
            token,
            message: message.into(),
            value,
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
        report(token.line, " at end", message);
    } else {
        let at = " at ".to_owned() + &token.lexeme + "'";
        report(token.line, &at, message);
    }
}

pub fn lox_runtime_error(_error: RuntimeError, had_runtime_error: &mut bool) {
    *had_runtime_error = true;
}
