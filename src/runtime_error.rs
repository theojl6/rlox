use crate::Token;

pub struct RuntimeError {
    token: Token,
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        eprintln!("{message}");

        return Self { token };
    }
}
