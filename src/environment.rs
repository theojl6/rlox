use std::collections::HashMap;

use crate::{error::RuntimeError, interpreter::Object, token::Token};

pub struct Environment {
    pub values: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, name: Token) -> Result<&Object, RuntimeError> {
        let val = self.values.get(&name.lexeme);
        match val {
            Some(o) => {
                return Ok(o);
            }
            None => Err(RuntimeError::new(
                name.clone(),
                &("Undefined variable '".to_owned() + &name.lexeme + "'."),
            )),
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }
        return Err(RuntimeError::new(
            name.clone(),
            &("Undefined variable '".to_owned() + &name.lexeme + "'."),
        ));
    }

    pub fn define(&mut self, name: String, value: Object) -> () {
        self.values.insert(name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn get_should_return_reference_if_exists() {
        let mut env = Environment {
            values: HashMap::new(),
        };
        let bool_obj = Object::Bool(true);
        env.values.insert(String::from("test_key"), bool_obj);
        let token = Token {
            line: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token);
        match obj {
            Ok(o) => {
                assert_eq!(*o, Object::Bool(true));
            }
            Err(_) => {
                panic!();
            }
        }
    }

    #[test]
    fn get_should_return_error_if_not_exists() {
        let env = Environment {
            values: HashMap::new(),
        };
        let token = Token {
            line: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token);
        assert!(obj.is_err());
    }
}
