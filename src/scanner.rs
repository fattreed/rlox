use std::{fmt, fs};

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

    // FIXME: not working with multiline
    #[must_use] pub fn scan_tokens(&self) -> Vec<Token> {
        let chars = self.source.chars(); 
        let mut tokens = vec![];
        for c in chars {
            let token_type = self.scan_token(c);
            println!("TOKEN {:?}", token_type);
            match token_type {
                Some(t) => {
                    tokens.push(Token {
                        token_type: t,
                        lexeme: String::new(),
                        literal: Some(Literal::new()),
                        line: self.line
                    });
                }
                _ => break,
            }
       }

        tokens.push(Token { 
            token_type: TokenType::EOF, 
            lexeme: String::new(), 
            literal: Some(Literal::new()), 
            line: self.line 
        });
        tokens
    }

    fn scan_token(&self, c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LEFT_PAREN),
            ')' => Some(TokenType::RIGHT_PAREN),
            '{' => Some(TokenType::LEFT_BRACE),
            '}' => Some(TokenType::RIGHT_BRACE),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '-' => Some(TokenType::MINUS),
            '+' => Some(TokenType::PLUS),
            ';' => Some(TokenType::SEMICOLON),
            '*' => Some(TokenType::STAR),
            '!' => {
                if c != '=' { 
                    Some(TokenType::BANG)
                } else {
                    Some(TokenType::BANG_EQUAL)
                }
            }
            '=' => {
                if c != '=' { 
                    Some(TokenType::EQUAL)
                } else {
                    Some(TokenType::EQUAL_EQUAL)
                }
            }
            '<' => {
                if c != '=' { 
                    Some(TokenType::LESS)
                } else {
                    Some(TokenType::LESS_EQUAL)
                }
            }
            '>' => {
                if c != '=' { 
                    Some(TokenType::GREATER)
                } else {
                    Some(TokenType::GREATER_EQUAL)
                }
            }
            _ => {
                eprint!("unexpected token: {}", self.line);
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {

}

impl Literal {
    fn new() -> Self {
        Self {}
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token { 
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize, //TODO: remove silenced warning
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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

#[test]
fn test_scan_token() {
    let source = fs::read_to_string("test.lox").expect("couldnt get file");
    let scanner = Scanner::new(source);
    let expected_tokens = vec![
        TokenType::LEFT_PAREN,
        TokenType::RIGHT_PAREN,
        TokenType::BANG,
        TokenType::EOF
    ];
   

    let tokens = scanner.scan_tokens();
    print!("{:?}", tokens);
    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type, expected_tokens[i]);
    }
}
