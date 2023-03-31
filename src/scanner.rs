use crate::lox::Error;
use std::fmt;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize, 
}

impl Scanner {
    #[must_use] pub const fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, error: &mut Error) -> Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token(error);
        }

        self.tokens.push(Token { 
            token_type: TokenType::EOF, 
            lexeme: String::new(), 
            literal: Some(Literal::new()), 
            line: self.line 
        });
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self, error: &mut Error) {
        let c = self.source.chars().nth(self.current);
        self.current += 1;

        match c {
            Some('(') => self.add_token(TokenType::LEFT_PAREN),
            Some(')') => self.add_token(TokenType::RIGHT_PAREN),
            Some('{') => self.add_token(TokenType::LEFT_BRACE),
            Some('}') => self.add_token(TokenType::RIGHT_BRACE),
            Some(',') => self.add_token(TokenType::COMMA),
            Some('.') => self.add_token(TokenType::DOT),
            Some('-') => self.add_token(TokenType::MINUS),
            Some('+') => self.add_token(TokenType::PLUS),
            Some(';') => self.add_token(TokenType::SEMICOLON),
            Some('*') => self.add_token(TokenType::STAR),
            Some('!') => {
                if self.is_at_end() || c != Some('='){ 
                    self.add_token(TokenType::BANG);
                } else {
                    self.add_token(TokenType::BANG_EQUAL);
                    self.current += 1;
                }
            }
            Some('=') => {
                if self.is_at_end() || c != Some('='){ 
                    self.add_token(TokenType::EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL_EQUAL);
                    self.current += 1;
                }
            }
            Some('<') => {
                if self.is_at_end() || c == Some('='){ 
                    self.add_token(TokenType::LESS);
                } else {
                    self.add_token(TokenType::LESS_EQUAL);
                    self.current += 1;
                }
            }
            Some('>') => {
                if self.is_at_end() || c == Some('='){ 
                    self.add_token(TokenType::GREATER);
                } else {
                    self.add_token(TokenType::GREATER_EQUAL);
                    self.current += 1;
                }
            }
            _ => error.error(self.line, "unexpected character"),
        };
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_literal(token, None);
    }

    fn add_token_literal(&mut self, token: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { 
            token_type: token, 
            lexeme: text.to_string(), 
            literal, 
            line: self.line 
        });
    }
}

#[derive(Debug, Clone)]
pub struct Literal {

}

impl Literal {
    fn new() -> Self {
        Self {

        } 
    }
}

#[derive(Debug, Clone)]
pub struct Token { 
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {
    // single char
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, 
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // one or two char tokens
    BANG, BANG_EQUAL, EQUAL,EQUAL_EQUAL, GREATER,
    GREATER_EQUAL, LESS, LESS_EQUAL,

    // literals
    IDENTIFIER, STRING, NUMBER,

    // keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

