#![allow(dead_code)]

mod token;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32, 
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while is_at_end() {
            start = current;
            scan_token()
        }

        tokens.push(Token { TokenType: EOF, lexeme: "", literal: None, line: line });
        tokens
    }

    fn isAtEnd(&self) -> bool {
        current >= source.char().count
    }

    fn scan_token(&self) {
        current += 1;
        let c = source.chars()[current];

        match c {
            Some('(') => self.add_token(LEFT_PAREN),
            Some(')') => self.add_token(RIGHT_PAREN),
            Some('{') => self.add_token(LEFT_BRACE),
            Some('}') => self.add_token(RIGHT_BRACE),
            Some(',') => self.add_token(COMMA),
            Some('.') => self.add_token(DOT),
            Some('-') => self.add_token(MINUS),
            Some('+') => self.add_token(PLUS),
            Some(';') => self.add_token(SEMICOLON),
            Some('*') => self.add_token(STAR),
            _ => break,
        };
    }

    fn add_token(&self, token: TokenType) {

    }
}

