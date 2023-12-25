use crate::{interpreter::Object, stmt::Stmt, token::Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Object),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(
        &mut self,
        ast_string: &mut String,
        name: &str,
        expression_strings: Vec<&String>,
    ) {
        ast_string.push('(');
        ast_string.push_str(name);
        for s in expression_strings {
            ast_string.push(' ');
            ast_string.push_str(s);
        }
        ast_string.push(')');
    }
}

impl AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        let mut ast = String::new();
        match e {
            Expr::Assign(_, _) => (),
            Expr::Binary(left, op, right) => {
                let left_expr = &self.visit_expr(left);
                let right_expr = &self.visit_expr(right);
                self.parenthesize(&mut ast, &op.lexeme, vec![left_expr, right_expr]);
            }
            Expr::Grouping(expr) => {
                let expr = &self.visit_expr(expr);
                self.parenthesize(&mut ast, &"group", vec![expr]);
            }
            Expr::Literal(o) => match o {
                Object::String(val) => {
                    ast.push_str(val);
                }
                Object::Number(val) => {
                    ast.push_str(&val.to_string());
                }
                Object::Bool(b) => {
                    ast.push_str(&b.to_string());
                }
                Object::Nil => {
                    ast.push_str(&"nil");
                }
            },
            Expr::Logical(..) => todo!(),
            Expr::Unary(op, expr) => {
                let expr = &self.visit_expr(expr);
                self.parenthesize(&mut ast, &op.lexeme, vec![expr]);
            }
            Expr::Variable(_) => todo!(),
        };
        ast
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn unary() {
        let mut ast_printer = AstPrinter;
        let unary_expression = Expr::Unary(
            Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Object::Number(0.0))),
        );
        assert_eq!(ast_printer.visit_expr(&unary_expression), "(- 0)")
    }

    #[test]
    fn binary() {
        let mut ast_printer = AstPrinter;
        let binary_expr = Expr::Binary(
            Box::new(Expr::Literal(Object::Number(1.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Object::Number(1.0))),
        );
        assert_eq!(ast_printer.visit_expr(&binary_expr), "(+ 1 1)")
    }

    #[test]
    fn grouping() {
        let mut ast_printer = AstPrinter;
        let grouping_expr = Expr::Grouping(Box::new(Expr::Literal(Object::String("hello".into()))));
        assert_eq!(ast_printer.visit_expr(&grouping_expr), "(group hello)");
    }

    #[test]
    fn binary_with_binary() {
        let mut ast_printer = AstPrinter;
        let binary_expr = Expr::Binary(
            Box::new(Expr::Literal(Object::Number(0.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Literal(Object::Number(1.0))),
        );
        let binary_expr_with_binary_expr = Expr::Binary(
            Box::new(Expr::Literal(Object::Number(0.0))),
            Token {
                token_type: TokenType::Plus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            Box::new(binary_expr),
        );

        assert_eq!(
            ast_printer.visit_expr(&binary_expr_with_binary_expr),
            "(- 0 (+ 0 1))"
        )
    }

    #[test]
    fn end_chapter_test() {
        let mut ast_printer = AstPrinter;
        let expression = Expr::Binary(
            Box::new(Expr::Unary(
                Token {
                    token_type: TokenType::Minus,
                    lexeme: String::from("-"),
                    literal: None,
                    line: 0,
                },
                Box::new(Expr::Literal(Object::Number(123.0))),
            )),
            Token {
                token_type: TokenType::Star,
                lexeme: String::from("*"),
                literal: None,
                line: 0,
            },
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Number(
                45.67,
            ))))),
        );
        assert_eq!(
            ast_printer.visit_expr(&expression),
            "(* (- 123) (group 45.67))"
        )
    }
}
