use crate::interpreter::Object;
use crate::report;
use crate::Token;

#[derive(Debug)]
pub struct RuntimeError {
    token: Token,
    message: String,
    pub value: Option<Object>,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str, value: Option<Object>) -> Self {
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
