use crate::scanner;
use std::{io, fs};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    #[must_use] pub const fn new() -> Self {
        Self { had_error: false }
    }
    //FIXME: fix this cause it's kinda not cool
    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(b) => {
                    // i think this is a windows bug bug the byte size is 2 when you pass in
                    // nothing. yes im using windows. edit: ok yea windows is weird
                    // im on fedora now, OK?!
                    // TODO: test on linux/mac
                    if b == 2 {
                        println!("bye pumpkin! bye pumpkin!");
                        break;
                    } 
                    self.run(line);
                    self.had_error = false;
                }
                Err(_) => break,
            }
        }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path);
        match contents {
            Ok(s) => self.run(s),
            Err(e) => eprintln!("{e}"),
        }

        if self.had_error {
            eprintln!("error!");
        }
    }

    pub fn run(&self, source: String) {
        let scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        for token in tokens {
            println!("{token:?}");
        }
    }
}

