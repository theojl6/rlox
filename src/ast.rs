use crate::{interpreter::Object, token::Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
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
            Expr::Assign { name, value } => (),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_expr = &self.visit_expr(left);
                let right_expr = &self.visit_expr(right);
                self.parenthesize(&mut ast, &operator.lexeme, vec![left_expr, right_expr]);
            }
            Expr::Call { .. } => todo!(),
            Expr::Grouping { expression } => {
                let expr = &self.visit_expr(expression);
                self.parenthesize(&mut ast, &"group", vec![expr]);
            }
            Expr::Literal { value } => match value {
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
            Expr::Logical { .. } => todo!(),
            Expr::Unary { operator, right } => {
                let expr = &self.visit_expr(right);
                self.parenthesize(&mut ast, &operator.lexeme, vec![expr]);
            }
            Expr::Variable { .. } => todo!(),
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
        let unary_expression = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(0.0),
            }),
        };
        assert_eq!(ast_printer.visit_expr(&unary_expression), "(- 0)")
    }

    #[test]
    fn binary() {
        let mut ast_printer = AstPrinter;
        let binary_expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
        assert_eq!(ast_printer.visit_expr(&binary_expr), "(+ 1 1)")
    }

    #[test]
    fn grouping() {
        let mut ast_printer = AstPrinter;
        let grouping_expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Object::String("hello".into()),
            }),
        };
        assert_eq!(ast_printer.visit_expr(&grouping_expr), "(group hello)");
    }

    #[test]
    fn binary_with_binary() {
        let mut ast_printer = AstPrinter;
        let binary_expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Object::Number(0.0),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
        let binary_expr_with_binary_expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Object::Number(0.0),
            }),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
            },
            right: Box::new(binary_expr),
        };

        assert_eq!(
            ast_printer.visit_expr(&binary_expr_with_binary_expr),
            "(- 0 (+ 0 1))"
        )
    }

    #[test]
    fn end_chapter_test() {
        let mut ast_printer = AstPrinter;
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: String::from("-"),
                    literal: None,
                    line: 0,
                },
                right: Box::new(Expr::Literal {
                    value: Object::Number(123.0),
                }),
            }),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: String::from("*"),
                literal: None,
                line: 0,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Object::Number(45.67),
                }),
            }),
        };
        assert_eq!(
            ast_printer.visit_expr(&expression),
            "(* (- 123) (group 45.67))"
        )
    }
}
