use std::{
    fs,
    io::{self, Write},
    net::TcpListener,
    process,
};

use ast::AstPrinter;
use error::{LoxError, RuntimeError};
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use server::handle_connection;
use token::{Token, TokenType};

pub mod ast;
pub mod class;
pub mod environment;
pub mod error;
pub mod function;
pub mod instance;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod server;
pub mod stmt;
pub mod token;

pub fn run_file<W: Write>(
    path: &str,
    writer: &mut W,
    had_error: &mut bool,
    had_runtime_error: &mut bool,
    debug_mode: bool,
) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    run(&contents, writer, had_error, had_runtime_error, debug_mode);
    if *had_error {
        process::exit(65);
    }
    if *had_runtime_error {
        process::exit(70);
    }
}

pub fn run_prompt<W: Write>(
    writer: &mut W,
    had_error: &mut bool,
    had_runtime_error: &mut bool,
    debug_mode: bool,
) {
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
        run(
            prompt.as_str(),
            writer,
            had_error,
            had_runtime_error,
            debug_mode,
        );
        *had_error = false;
    }
}

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn run<W: Write>(
    source: &str,
    writer: &mut W,
    had_error: &mut bool,
    _had_runtime_error: &mut bool,
    debug_mode: bool,
) {
    let mut scanner = Scanner::new(String::from(source));
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    match stmts {
        Ok(stmts) => {
            let mut interpreter = Interpreter::new(writer);
            if debug_mode {
                let mut ast_printer = AstPrinter;
                ast_printer.print(stmts.clone());
            }
            let mut resolver = Resolver::new(interpreter);
            if let Err(e) = resolver.resolve_stmts(&stmts) {
                e.report();
                *had_error = true;
            }
            interpreter = resolver.interpreter;
            interpreter.interpret(&stmts);
        }
        Err(_e) => {
            *had_error = true;
        }
    }
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
