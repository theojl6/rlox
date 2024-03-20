use std::collections::HashMap;

use crate::ast::Expr;
use crate::ast::Visitor;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(&self, interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.visit_stmt(statement)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), false);
        self.scopes.push(scope);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }
}

impl Visitor<(), ()> for Resolver {
    fn visit_expr(&mut self, e: &Expr) -> Result<(), RuntimeError> {
        match e {
            Expr::Assign { name, value } => todo!(),
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
            Expr::Grouping { expression } => todo!(),
            Expr::Literal { value } => todo!(),
            Expr::Logical {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Unary { operator, right } => todo!(),
            Expr::Variable { name } => {
                if !self.scopes.is_empty()
                    && self
                        .scopes
                        .last()
                        .unwrap()
                        .get(&name.lexeme)
                        .is_some_and(|b| *b == false)
                {
                    return Err(RuntimeError::new(
                        name.clone(),
                        "Can't read local variable in its own initializer.",
                        None,
                    ));
                }
                self.resolve_local(e, name);
                Ok(())
            }
        }
    }

    fn visit_stmt(&mut self, s: &crate::stmt::Stmt) -> Result<(), RuntimeError> {
        match s {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
                Ok(())
            }
            Stmt::Class {
                name,
                superclass,
                methods,
            } => todo!(),
            Stmt::Expr(_) => todo!(),
            Stmt::Function { name, params, body } => todo!(),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::Print(_) => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if let Some(i) = initializer {
                    self.visit_expr(&i)?;
                }
                self.define(name);
                Ok(())
            }
            Stmt::While { condition, body } => todo!(),
        }
    }
}
