use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::Object, token::Token};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Rc<RefCell<Object>>>,
}
impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: Token) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(o) => Ok(*o),
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
        let enclosing = self.enclosing.to_owned().expect("no enclosing environment");
        let mut environment = enclosing;
        // println!("looking up ancestor at distance {}", distance);
        for _ in 0..(distance - 1) {
            let enclosing = environment
                .borrow_mut()
                .enclosing
                .to_owned()
                .expect("no enclosing environment");
            environment = enclosing;
            println!("hopped ancestor");
        }
        environment
    }

    pub fn get_at(
        &self,
        distance: usize,
        name: String,
    ) -> Result<Rc<RefCell<Object>>, RuntimeError> {
        // println!("get_at distance: {}", distance);
        // println!("get_at name: {}", name);
        if distance == 0 {
            println!("distance is 0, getting from self");
            return Ok(self.values.get(&name).unwrap().clone());
        } else {
            println!("distance is some, getting from ancestor");
            println!("trying to get {} at distance {}", name, distance);
            let ancestor = self.ancestor(distance);
            let ancestor = ancestor.borrow_mut();
            println!("ancestor at distance {}: {:?}", distance, ancestor);
            let object = ancestor.values.get(&name);
            if let Some(o) = object {
                return Ok(o.clone());
            } else {
                println!("cannot get object");
            }
        }
        panic!()
    }

    pub fn assign_at(&mut self, distance: usize, name: Token, value: Rc<RefCell<Object>>) {
        self.ancestor(distance)
            .borrow_mut()
            .values
            .insert(name.lexeme, value);
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
                assert_eq!(o, bool_obj.clone());
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

    #[test]
    fn should_resolve_if_variable_is_in_enclosing_environment() {
        let mut enclosing = Environment::new(None);
        let bool_obj = Rc::new(RefCell::new(Object::Bool(true)));
        enclosing.values.insert(String::from("test_key"), bool_obj);
        let env = Environment::new(Some(Rc::new(RefCell::new(enclosing))));
        let token = Token {
            line: 0,
            lexeme: String::from("test_key"),
            literal: None,
            token_type: TokenType::Identifier,
        };
        let obj = env.get(token).expect("Cannot find variable");
        assert_eq!(obj, bool_obj.clone());
    }
}
