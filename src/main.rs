use std::env;
use rlox::lox::{Lox, LoxError};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut lox = Lox { error: LoxError { had_error: false } };

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        let path = &args[1];
        lox.run_file(path.to_string());
    } else {
        lox.run_prompt()
    }
}

