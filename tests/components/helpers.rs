use std::{fs, io::Write};

use rlox::{interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner};

pub fn test_file<W: Write + 'static>(
    path: &str,
    writer: W,
    had_error: &mut bool,
    had_runtime_error: &mut bool,
) -> W {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    test_run(&contents, writer, had_error, had_runtime_error)
}

pub fn test_run<W: Write + 'static>(
    source: &str,
    writer: W,
    had_error: &mut bool,
    _had_runtime_error: &mut bool,
) -> W {
    let mut scanner = Scanner::new(String::from(source));
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens, writer);
    let stmts = parser.parse();
    match stmts {
        Ok(stmts) => {
            let mut interpreter = Interpreter::new(parser.writer);
            let mut resolver = Resolver::new(interpreter);
            if let Err(e) = resolver.resolve_stmts(&stmts) {
                *had_error = true;
            }
            interpreter = resolver.interpreter;
            interpreter.interpret(&stmts);
            interpreter.writer
        }
        Err(_e) => {
            *had_error = true;
            parser.writer
        }
    }
}
