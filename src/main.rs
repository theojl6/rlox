use rlox::run_file;
use rlox::run_prompt;
use std::env;

fn main() {
    let mut had_error = false;
    let mut had_runtime_error = false;
    let args: Vec<String> = env::args().collect();
    let debug_mode = env::var("DEBUG").is_ok();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1], &mut had_error, &mut had_runtime_error, debug_mode);
    } else {
        run_prompt(&mut had_error, &mut had_runtime_error, debug_mode);
    }
}
