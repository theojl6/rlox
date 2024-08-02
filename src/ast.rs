use std::hash::{Hash, Hasher};

use crate::{error::RuntimeError, interpreter::Object, stmt::Stmt, token::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Get {
        object: Box<Expr>,
        name: Token,
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
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Assign { name, value } => {
                name.hash(state);
                value.hash(state);
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                left.hash(state);
                operator.hash(state);
                right.hash(state);
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                callee.hash(state);
                paren.hash(state);
                arguments.hash(state);
            }
            Expr::Get { object, name } => {
                object.hash(state);
                name.hash(state);
            }
            Expr::Grouping { expression } => {
                expression.hash(state);
            }
            Expr::Literal { value } => {
                value.hash(state);
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                left.hash(state);
                operator.hash(state);
                right.hash(state);
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                object.hash(state);
                name.hash(state);
                value.hash(state);
            }
            Expr::This { keyword } => keyword.hash(state),
            Expr::Unary { operator, right } => {
                operator.hash(state);
                right.hash(state);
            }
            Expr::Variable { name } => name.hash(state),
        }
    }
}

pub trait Visitor<T, K> {
    fn visit_expr(&mut self, e: &Expr) -> Result<T, RuntimeError>;

    fn visit_stmt(&mut self, s: &Stmt) -> Result<K, RuntimeError>;
}
pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(
        &mut self,
        ast_string: &mut String,
        name: &str,
        expression_strings: Vec<String>,
    ) {
        ast_string.push('(');
        ast_string.push_str(name);
        for s in expression_strings {
            ast_string.push(' ');
            ast_string.push_str(&s);
        }
        ast_string.push(')');
    }

    pub fn print(&mut self, stmts: Vec<Stmt>) {
        for s in stmts {
            let string = self.visit_stmt(&s);
            match string {
                Ok(string) => {
                    println!("{}", string)
                }
                Err(_) => {
                    println!("Error printing statement")
                }
            }
        }
    }
}

impl Visitor<String, String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> Result<String, RuntimeError> {
        let mut ast = String::new();
        match e {
            Expr::Assign { name, value } => {
                let expr = self.visit_expr(value)?;
                self.parenthesize(&mut ast, &"assign", vec![name.lexeme.clone(), expr]);
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_expr = self.visit_expr(left)?;
                let right_expr = self.visit_expr(right)?;
                self.parenthesize(&mut ast, &operator.lexeme, vec![left_expr, right_expr]);
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let callee = &self.visit_expr(callee)?;
                let arguments: Vec<String> = arguments
                    .iter()
                    .map(|e| self.visit_expr(e).expect("error visiting expressions"))
                    .collect();

                self.parenthesize(&mut ast, callee, arguments)
            }
            Expr::Get { object, name } => {
                let value = self.visit_expr(object)?;
                self.parenthesize(&mut ast, &"get", vec![name.lexeme.clone(), value]);
            }
            Expr::Grouping { expression } => {
                let expr = self.visit_expr(expression)?;
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
                Object::Class(_c) => ast.push_str(&"Class"),
                Object::Instance(_i) => ast.push_str(&"Instance"),
                Object::Nil => {
                    ast.push_str(&"nil");
                }
                Object::Function(func) => {
                    let declaration = &func.declaration;
                    let name = if let Stmt::Function { name, .. } = declaration {
                        name.lexeme.clone()
                    } else {
                        String::from("<unnamed>")
                    };

                    ast.push_str(&(format!("<fun>{}", &name)));
                }
                Object::NativeFunction(..) => {
                    ast.push_str(&"<native fun>");
                }
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_expr = self.visit_expr(left)?;
                let right_expr = self.visit_expr(right)?;
                self.parenthesize(&mut ast, &operator.lexeme, vec![left_expr, right_expr]);
            }
            Expr::Set {
                object: _,
                name: _,
                value: _,
            } => {
                todo!()
            }
            Expr::This { keyword: _ } => todo!(),
            Expr::Unary { operator, right } => {
                let expr = self.visit_expr(right)?;
                self.parenthesize(&mut ast, &operator.lexeme, vec![expr]);
            }
            Expr::Variable { name } => ast.push_str(&name.lexeme),
        };
        Ok(ast)
    }

    fn visit_stmt(&mut self, s: &Stmt) -> Result<String, RuntimeError> {
        let mut ast = String::new();
        match s {
            Stmt::Block { statements } => {
                ast.push_str("{\n");
                for s in statements {
                    let stmt = self.visit_stmt(s)?;
                    ast.push_str(&format!("  {};\n", &stmt));
                }
                ast.push_str("}");
            }
            Stmt::Class {
                name: _,
                methods: _,
            } => todo!(),
            Stmt::Expr(e) => {
                let expr = self.visit_expr(e)?;
                ast.push_str(&expr)
            }
            Stmt::Function { name, params, body } => {
                let mut function = String::new();
                function.push_str(&format!("fun {}(", &name.lexeme));

                let params = params
                    .iter()
                    .map(|p| p.lexeme.clone())
                    .collect::<Vec<String>>()
                    .join(", ");

                function.push_str(&params);

                function.push_str(")");

                function.push_str(" {\n");

                let body: String = body
                    .iter()
                    .map(|b| {
                        format!(
                            "    {};\n",
                            &self.visit_stmt(b).expect("error printing function body")
                        )
                    })
                    .collect();

                function.push_str(&body);

                function.push_str("}");

                ast.push_str(&function);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let c = self.visit_expr(condition)?;
                let then = self.visit_stmt(&then_branch)?;

                ast.push_str(format!("if ({c}) {{ {then} }}").as_str());

                if let Some(b) = else_branch {
                    let else_b = self.visit_stmt(&b)?;
                    ast.push_str(&format!(" else {{ {} }}", &else_b));
                }
            }
            Stmt::Print(e) => {
                let expr = &self.visit_expr(e)?;
                ast.push_str(&format!("print {expr};"));
            }
            Stmt::Return { keyword, value } => {
                ast.push_str(&format!("{} {}", keyword.lexeme, &self.visit_expr(value)?));
            }
            Stmt::Var { name, initializer } => {
                ast.push_str(&name.lexeme.clone());
                if let Some(i) = initializer {
                    ast.push_str(&(" = ".to_owned() + &self.visit_expr(i)?));
                }
            }
            Stmt::While { condition, body } => {
                let c = self.visit_expr(condition)?;
                let b = self.visit_stmt(&body)?;

                ast.push_str(&("while (".to_owned() + &c + ") { " + &b + " }"));
            }
        }
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn unary_expression() {
        let mut ast_printer = AstPrinter;
        let unary_expression = Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
                position: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(0.0),
            }),
        };
        assert_eq!(
            ast_printer.visit_expr(&unary_expression).expect(""),
            "(- 0)"
        )
    }

    #[test]
    fn unary_expression_statement() {
        let mut ast_printer = AstPrinter;
        let unary_stmt_expr = Stmt::Expr(Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 0,
                position: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(0.0),
            }),
        });
        ast_printer.print(vec![unary_stmt_expr])
    }

    #[test]
    fn binary_expression() {
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
                position: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Number(1.0),
            }),
        };
        assert_eq!(ast_printer.visit_expr(&binary_expr).expect(""), "(+ 1 1)")
    }

    #[test]
    fn grouping_expression() {
        let mut ast_printer = AstPrinter;
        let grouping_expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Object::String("hello".into()),
            }),
        };
        assert_eq!(
            ast_printer.visit_expr(&grouping_expr).expect(""),
            "(group hello)"
        );
    }

    #[test]
    fn variable_expression() {
        let mut ast_printer = AstPrinter;
        let variable_expr = Expr::Variable {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("x"),
                literal: None,
                line: 0,
                position: 0,
            },
        };
        assert_eq!(ast_printer.visit_expr(&variable_expr).expect(""), "x")
    }

    #[test]
    fn binary_with_binary_expression() {
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
                position: 0,
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
                position: 0,
            },
            right: Box::new(binary_expr),
        };

        assert_eq!(
            ast_printer
                .visit_expr(&binary_expr_with_binary_expr)
                .expect(""),
            "(- 0 (+ 0 1))"
        )
    }

    #[test]
    fn logical_expression() {
        let mut ast_printer = AstPrinter;
        let logical_expr = Expr::Logical {
            left: Box::new(Expr::Literal {
                value: Object::Bool(true),
            }),
            operator: Token {
                token_type: TokenType::And,
                lexeme: String::from("and"),
                literal: None,
                line: 0,
                position: 0,
            },
            right: Box::new(Expr::Literal {
                value: Object::Bool(true),
            }),
        };

        assert_eq!(
            ast_printer.visit_expr(&logical_expr).expect(""),
            "(and true true)"
        )
    }

    #[test]
    fn assign_expression() {
        let mut ast_printer = AstPrinter;
        let assign_expr = Expr::Assign {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: String::from("x"),
                literal: None,
                line: 0,
                position: 0,
            },
            value: Box::new(Expr::Literal { value: Object::Nil }),
        };

        assert_eq!(
            ast_printer.visit_expr(&assign_expr).expect(""),
            "(assign x nil)"
        )
    }

    #[test]
    fn call_expression() {
        let mut ast_printer = AstPrinter;
        let call_expr = Expr::Call {
            callee: Box::new(Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: String::from("hello"),
                    literal: None,
                    line: 0,
                    position: 0,
                },
            }),
            paren: Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                literal: None,
                line: 0,
                position: 0,
            },
            arguments: vec![
                Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: Object::Number(0.0),
                    }),
                    operator: Token {
                        token_type: TokenType::Plus,
                        lexeme: String::from("+"),
                        literal: None,
                        line: 0,
                        position: 0,
                    },
                    right: Box::new(Expr::Literal {
                        value: Object::Number(1.0),
                    }),
                }),
                Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: Object::Number(1.0),
                    }),
                    operator: Token {
                        token_type: TokenType::Plus,
                        lexeme: String::from("-"),
                        literal: None,
                        line: 0,
                        position: 0,
                    },
                    right: Box::new(Expr::Literal {
                        value: Object::Number(1.0),
                    }),
                }),
            ],
        };
        assert_eq!(
            ast_printer.visit_expr(&call_expr).expect(""),
            "(hello (+ 0 1) (- 1 1))"
        );
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
                    position: 0,
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
                position: 0,
            },
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Object::Number(45.67),
                }),
            }),
        };
        assert_eq!(
            ast_printer.visit_expr(&expression).expect(""),
            "(* (- 123) (group 45.67))"
        )
    }
}
