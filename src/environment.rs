use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::Object, token::Token};

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        let val = self.values.get(&name.lexeme);
        match val {
            Some(o) => Ok(o.clone()),
            None => match &self.enclosing {
                Some(e) => {
                    let enc = e.borrow();
                    enc.get(name)
                }
                None => Err(RuntimeError::new(
                    name.clone(),
                    &("Undefined variable '".to_owned() + &name.lexeme + "'."),
                )),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        match &self.enclosing {
            Some(e) => {
                e.borrow_mut().assign(name, value)?;
                Ok(())
            }
            None => {
                return Err(RuntimeError::new(
                    name.clone(),
                    &("Undefined variable '".to_owned() + &name.lexeme + "'."),
                ));
            }
        }
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
        let mut env = Environment::new(None);
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
                assert_eq!(o, Object::Bool(true));
            }
            Err(_) => {
                panic!();
            }
        }
    }

    #[test]
    fn get_should_return_error_if_not_exists() {
        let env = Environment::new(None);
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
