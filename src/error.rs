use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::Object;
use crate::report;
use crate::Token;

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
