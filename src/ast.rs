use crate::token::{Literal, Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Literal),
}

pub trait Visitor {
    fn walk_expr(&mut self, e: &Expr);
}

pub struct Interpretor;

impl Visitor for Interpretor {
    fn walk_expr(&mut self, e: &Expr) {
        match e {
            Expr::Assign(_, _) => {
                println!("this assign");
            }
            Expr::Binary(left, _, right) => {
                self.walk_expr(&left);
                self.walk_expr(&right);
            }
            Expr::Literal(_) => {
                println!("this literal");
            }
        }
    }
}

pub struct AstPrinter {
    pub string: String,
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {
            string: String::new(),
        }
    }
    pub fn print(&mut self, e: &Expr) {
        self.string = String::new();
        self.walk_expr(e);
        println!("{}", self.string);
    }
}

impl Visitor for AstPrinter {
    fn walk_expr(&mut self, e: &Expr) {
        match e {
            Expr::Assign(_, _) => {}
            Expr::Binary(left, op, right) => {
                self.string.push('(');
                self.string.push_str(&op.lexeme);
                self.string.push(' ');
                self.walk_expr(left);
                self.string.push(' ');
                self.walk_expr(right);
                self.string.push(')');
            }
            Expr::Literal(literal) => match literal {
                Literal::String(val) => {
                    self.string.push_str(val);
                }
                Literal::Number(val) => {
                    self.string.push_str(&val.to_string());
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    use super::*;

    #[test]
    fn binary_expression() {
        let mut ast_printer = AstPrinter::new();
        let expression_1 = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(0.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Literal::Number(1.0))),
        );
        let expression_2 = Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(0.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            Box::new(expression_1),
        );

        ast_printer.print(&expression_2);
        assert_eq!(ast_printer.string, "(- 0 (+ 0 1))")
    }
}
