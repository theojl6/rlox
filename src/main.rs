use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::usize;

#[derive(Debug)]
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

#[derive(Debug)]
struct Literal {}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
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
    let mut scanner = Scanner::new(String::from(source));
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_single_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: Literal {},
            line: self.line,
        });
        &self.tokens
    }
    fn is_at_end(&mut self) -> bool {
        usize::try_from(self.current)
            .map(|current| current >= self.source.len())
            .unwrap_or(true)
    }
    fn scan_single_token(&mut self) {
        let c = self.advance();
        println!("single token: {:?}", &c);
        match c {
            Some('(') => self.add_token(TokenType::LeftParen, Literal {}),
            Some(')') => self.add_token(TokenType::RightParen, Literal {}),
            Some('{') => self.add_token(TokenType::LeftBrace, Literal {}),
            Some('}') => self.add_token(TokenType::RightBrace, Literal {}),
            Some(',') => self.add_token(TokenType::Comma, Literal {}),
            Some('.') => self.add_token(TokenType::Dot, Literal {}),
            Some('-') => self.add_token(TokenType::Minus, Literal {}),
            Some('+') => self.add_token(TokenType::Plus, Literal {}),
            Some(';') => self.add_token(TokenType::Semicolon, Literal {}),
            Some('*') => self.add_token(TokenType::Star, Literal {}),
            Some(_) => error(self.line, String::from("Unexpected character.")),
            None => (),
        }
    }
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current = self.current + 1;
        c
    }
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: String::from(text),
            literal,
            line: self.line,
        });
    }
}

fn error(line: usize, message: String) {
    report(line, String::from(""), message);
}

fn report(line: usize, at: String, message: String) {
    println!("[line {line}] Error {at}: {message}");
}
