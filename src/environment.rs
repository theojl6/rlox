use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::Object, token::Token};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Rc<RefCell<Object>>>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, name: Token) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(o) => Ok(Rc::clone(o)),
            None => match &self.enclosing {
                Some(e) => {
                    let enc = e.borrow_mut();
                    enc.get(name)
                }
                None => Err(RuntimeError::new(
                    name.clone(),
                    &("Get: Undefined variable '".to_owned() + &name.lexeme + "'."),
                    None,
                )),
            },
        }
    }

    pub fn assign(&mut self, name: Token, value: Rc<RefCell<Object>>) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return Ok(());
        }

        match &self.enclosing {
            Some(e) => {
                e.as_ref().borrow_mut().assign(name, value)?;
                Ok(())
            }
            None => Err(RuntimeError::new(
                name.clone(),
                &("Assign: Undefined variable '".to_owned() + &name.lexeme + "'."),
                None,
            )),
        }
    }

    pub fn define(&mut self, name: String, value: Rc<RefCell<Object>>) -> () {
        self.values.insert(name, value);
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let enclosing = Rc::clone(&self.enclosing.as_ref().unwrap());
        let mut environment = enclosing;
        for _ in 1..distance {
            let enclosing = Rc::clone(&environment.borrow().enclosing.as_ref().unwrap());
            environment = enclosing;
        }
        environment
    }

    pub fn get_at(
        &self,
        distance: usize,
        name: String,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        if distance == 0 {
            return Ok(Rc::clone(self.values.get(&name).unwrap()));
        } else {
            println!("[ENVIRONMENT] current environment: {:?}", self);
            let ancestor = self.ancestor(distance);
            let ancestor = ancestor.borrow_mut();
            println!(
                "[ENVIRONMENT] got ancestor {:?} at distance {:?}",
                ancestor, distance
            );
            let object = ancestor.values.get(&name);
            if let Some(o) = object {
                return Ok(Rc::clone(o));
            } else {
                println!("[ENVIRONMENT] cannot get object");
            }
        }
        panic!()
    }

    pub fn assign_at(&mut self, distance: usize, name: Token, value: Rc<RefCell<Object>>) {
        if distance == 0 {
            self.values.insert(name.lexeme, value);
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn get_should_return_reference_if_exists() {
        let mut env = Environment::new(None);
        let bool_obj = Rc::new(RefCell::new(Object::Bool(true)));
        env.values
            .insert(String::from("test_key"), Rc::clone(&bool_obj));
        let token = Token {
            line: 0,
            position: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token);
        match obj {
            Ok(o) => {
                assert_eq!(*o.borrow(), *bool_obj.borrow());
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
            position: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token);
        assert!(obj.is_err());
    }

    #[test]
    fn should_resolve_if_variable_is_in_enclosing_environment() {
        let mut enclosing = Environment::new(None);
        let bool_obj = Rc::new(RefCell::new(Object::Bool(true)));
        enclosing
            .values
            .insert(String::from("test_key"), Rc::clone(&bool_obj));
        let env = Environment::new(Some(Rc::new(RefCell::new(enclosing))));
        let token = Token {
            line: 0,
            position: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token).expect("Cannot find variable");
        assert_eq!(obj, bool_obj.clone());
    }
}
