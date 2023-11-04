use crate::report;
use crate::Token;

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        report(token.line, &token.lexeme, &message);

        return Self {
            token,
            message: message.into(),
        };
    }
}
