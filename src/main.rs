use std::env;
use std::fs;
use std::io;
use std::process;
use std::usize;

mod ast;
mod interpreter;
mod parser;
mod runtime_error;
mod scanner;
mod token;
use token::Token;
use token::TokenType;

use crate::ast::AstPrinter;
use crate::ast::Visitor;
use crate::interpreter::Interpretor;
use crate::parser::Parser;
use crate::runtime_error::RuntimeError;
use crate::scanner::Scanner;

fn main() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1], &mut had_error, &mut had_runtime_error);
    } else {
        run_prompt(&mut had_error, &mut had_runtime_error);
    }
}

fn run_file(path: &str, had_error: &mut bool, had_runtime_error: &mut bool) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    run(&contents, had_error, had_runtime_error);
    if *had_error {
        process::exit(65);
    }
    if *had_runtime_error {
        process::exit(70);
    }
}

fn run_prompt(had_error: &mut bool, had_runtime_error: &mut bool) {
    loop {
        let mut prompt = String::new();
        println!("> ");
        io::stdin()
            .read_line(&mut prompt)
            .expect("failed to read line");
        prompt = prompt.trim().to_string();
        if prompt == "exit" || prompt == "" {
            break;
        }
        run(prompt.as_str(), had_error, had_runtime_error);
        *had_error = false;
    }
}

fn run(source: &str, had_error: &mut bool, had_runtime_error: &mut bool) {
    let mut scanner = Scanner::new(String::from(source));
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse();
    match expr {
        Ok(expr) => {
            let mut ast_printer = AstPrinter;
            println!("{}", ast_printer.visit_expr(&expr));
            let mut interpreter = Interpretor;
            let value = interpreter.interpret(&expr);
            match value {
                Ok(v) => {
                    println!("{v}")
                }
                Err(e) => lox_runtime_error(e, had_runtime_error),
            }
        }
        Err(e) => {
            *had_error = true;
        }
    }
}

fn error(line: usize, message: &str) {
    report(line, &"", message);
}

pub fn report(line: usize, at: &str, message: &str) {
    println!("[line {line}] Error {at}: {message}");
}

pub fn lox_error(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        let at = " at ".to_owned() + &token.lexeme + "'";
        report(token.line, &at, message);
    }
}

pub fn lox_runtime_error(error: RuntimeError, had_runtime_error: &mut bool) {
    *had_runtime_error = true;
}
