use crate::ast::Expr;
use crate::ast::Visitor;
use crate::error::RuntimeError;
use crate::interpreter::Interpreter;

pub struct Resolver {
    interpreter: Interpreter,
}

impl Visitor<(), ()> for Resolver {
    fn visit_expr(&mut self, e: &Expr) -> Result<(), RuntimeError> {
        todo!()
    }

    fn visit_stmt(&mut self, s: &crate::stmt::Stmt) -> Result<(), RuntimeError> {
        todo!()
    }
}
