use std::env;
use std::fs;
use std::io;
use std::usize;

mod ast;
mod parser;
mod scanner;
mod token;
use crate::scanner::Scanner;

fn main() {
    let mut had_error = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1], had_error);
    } else {
        run_prompt(&mut had_error);
    }
}

fn run_file(path: &str, had_error: bool) {
    if had_error {
        panic!();
    }
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    run(contents.as_str());
}

fn run_prompt(had_error: &mut bool) {
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
        run(prompt.as_str());
        *had_error = false;
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(String::from(source));
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: String) {
    report(line, String::from(""), message);
}

fn report(line: usize, at: String, message: String) {
    println!("[line {line}] Error {at}: {message}");
}
