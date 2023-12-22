use crate::{ast::Expr, token::Token};

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Expr,
        methods: Vec<Stmt>,
    },
    Expr(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print(Expr),
    Return {
        keyword: Token,
        value: Expr,
    },
    Var {
        name: Token,
        intializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
