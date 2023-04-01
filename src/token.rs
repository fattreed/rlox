use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    None,
}

#[derive(Debug, Clone)]
pub struct Token { 
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[allow(non_camel_case_types, clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

