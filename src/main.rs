use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    run(contents.as_str());
}

fn run_prompt() {
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
    }
}

fn run(source: &str) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

struct Scanner<'a> {
    source: &'a str,
}

impl<'a> Scanner<'a> {
    fn scan_tokens(&'a self) -> Vec<&'a str> {
        vec![self.source]
    }
}
