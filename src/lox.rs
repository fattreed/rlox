use crate::scanner;
use std::{io, fs};

pub struct Lox {
    pub error: LoxError,
}

impl Lox {
    //REPL
    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(b) => {
                    // i think this is a windows bug bug the byte size is 2 when you pass in
                    // nothing. yes im using windows.
                    // TODO: test on linux/mac
                    if b == 2 {
                        println!("bye pumpkin! bye pumpkin!");
                        break;
                    } else {
                        self.run(line);
                        self.error.had_error = false;
                    }
                }
                Err(_) => break,
            }
        }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path)
            .expect("cannot read from file");
        self.run(contents);
        if self.error.had_error {
            eprintln!("error!");
        }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens(&mut self.error);
        
        for token in tokens {
            println!("{:?}", token);
        }
    }
}

pub struct LoxError {
    pub had_error: bool,
}

impl LoxError {
    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!("line: {}. Error {}: {}", line, location, message);
        self.had_error = true;
    }
}

#[test]
fn test_run_file() {
    let _path = "test.lox";
    
}
