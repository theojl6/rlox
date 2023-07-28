use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::usize;

pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

struct Literal {}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32,
}

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
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<TokenType>,
    start: i32,
    current: i32,
    line: i32,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&'a mut self) -> &Vec<TokenType> {
        while !self.is_at_end() {
            self.start = self.current;
        }
        &self.tokens
    }
    fn is_at_end(&'a self) -> bool {
        usize::try_from(self.current)
            .map(|current| current > self.source.len())
            .unwrap_or(false)
    }
}

fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: i32, at: String, message: String) {
    println!("[line {line}] Error {at}: {message}");
}
