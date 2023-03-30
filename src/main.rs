#![allow(dead_code)]

use std::{fs, env, io};
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut lox = Lox { had_error: false };

    if args.iter().count() > 2 {
        println!("Usage: rlox [script]");
    } else if args.iter().count() == 2 {
        let path = &args[1];
        lox.run_file(path.to_string());
    } else {
        lox.run_prompt()
    }
}

struct Lox {
    had_error: bool
}

impl Lox {
    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(b) => {
                    if b == 2 {
                        println!("bye pumpkin! bye pumpkin!");
                        break;
                    } else {
                        self.run(line);
                        self.had_error = false;
                    }
                }
                Err(_) => break,
            }
        }
    }

    fn run_file(&self, path: String) {
        let contents = fs::read_to_string(path)
            .expect("cannot read from file");
        self.run(contents);
        if self.had_error {
            eprintln!("error!");
        }
    }

    fn run(&self, source: String) {
        let tokens = scanner::scan_tokens(source);
        
        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn error(&mut self, line: i32, message: String) {
        self.report(line, String::new(), message);
    }

    fn report(&mut self, line: i32, location: String, message: String) {
        eprintln!("line: {}. Error {}: {}", line, location, message);
        self.had_error = true
    }

}


#[test]
fn test_run_file() {
    let path = "test.lox";
    
}
