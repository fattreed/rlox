use std::env;
use std::cmp::Ordering;
use rlox::lox::{Lox, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut lox = Lox { error: Error { had_error: false } };

    match args.len().cmp(&2) {
        Ordering::Greater => println!("Usage: rlox [script]"),
        Ordering::Less => lox.run_prompt(),
        Ordering::Equal => {
            let path = &args[1];
            lox.run_file(path.to_string());
        }
    }
}

