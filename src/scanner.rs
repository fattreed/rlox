use std::fmt;

#[derive(Debug)]
pub struct Scanner {
    source: String,
}

impl Scanner {
    #[must_use] pub const fn new(source: String) -> Self {
        Self {
            source,
        }
    }

    #[must_use] pub fn scan_tokens(&self) -> Vec<Token> {
        let chars: Vec<_> = self.source.chars().collect(); 
        let mut tokens = vec![];
        let mut current: usize = 0;
        let mut start: usize = current;
        let mut line: usize = 1;
        for c in chars.iter() {
            start = current;
            
            let is_at_end = current >= chars.iter().count();
            let token_type = self.scan_token(&mut current, 
                                             &mut line, 
                                             is_at_end);
            
            let text = ""; 

            match token_type {
                Some(t) => tokens.push(self.create_token(t, text, line)), 
                _ => (),
            }
        }
        tokens.push(self.create_token_literal(TokenType::EOF, "", None, line));
        tokens
    }

    fn create_token(&self, 
                    token_type: TokenType, 
                    text: &str, 
                    line: usize) -> Token {
        self.create_token_literal(token_type, text, None, line)
    }

    fn create_token_literal(&self, 
                            token_type: TokenType, 
                            text: &str,
                            literal: Option<Literal>,
                            line: usize) -> Token {
        Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: line,
        } 
    }

    fn scan_token(&self, 
                  current: &mut usize,
                  line: &mut usize, 
                  is_at_end: bool) -> Option<TokenType> {
        let c = self.advance(current);
        match c {
            Some('(') => Some(TokenType::LEFT_PAREN),
            Some(')') => Some(TokenType::RIGHT_PAREN),
            Some('{') => Some(TokenType::LEFT_BRACE),
            Some('}') => Some(TokenType::RIGHT_BRACE),
            Some(',') => Some(TokenType::COMMA),
            Some('.') => Some(TokenType::DOT),
            Some('-') => Some(TokenType::MINUS),
            Some('+') => Some(TokenType::PLUS),
            Some(';') => Some(TokenType::SEMICOLON),
            Some('*') => Some(TokenType::STAR),
            Some('!') => {
                if self.check_next_char(is_at_end, current, '=') {
                    Some(TokenType::BANG_EQUAL)
                } else {
                    Some(TokenType::BANG)
                }
            }
            Some('=') => {
                if self.check_next_char(is_at_end, current, '=') {
                    Some(TokenType::EQUAL_EQUAL)
                } else {
                    Some(TokenType::EQUAL)
                }
            }
            Some('<') => {
                if self.check_next_char(is_at_end, current, '=') {
                    Some(TokenType::LESS_EQUAL)
                } else {
                    Some(TokenType::LESS)
                }
            }
            Some('>') => {
                if self.check_next_char(is_at_end, current, '=') {
                    Some(TokenType::GREATER_EQUAL)
                } else {
                    Some(TokenType::GREATER)
                }
            }
            Some('/') => {
                if self.check_next_char(is_at_end, current, '/') {
                    while self.peek(*current, is_at_end) != Some('\n') && !is_at_end {
                        _ = self.advance(current);
                    }
                    None
                } else {
                    Some(TokenType::SLASH)
                }
            }
            Some(' ') | Some('\r') | Some('\t') => None,
            Some('\n') => { *line += 1; None }
            _ => {
                eprint!("unexpected token: {}", 0);
                None
            }
        }
    }

    fn advance(&self, current: &mut usize) -> Option<char> {
        let prev = current.clone();
        *current += 1;
        self.source.chars().nth(prev)
    }

    fn peek(&self, current: usize, is_at_end: bool) -> Option<char> {
        if is_at_end { return Some('\0') }
        self.source.chars().nth(current)
    }

    fn check_next_char(&self,
                       is_at_end: bool, 
                       current: &mut usize, 
                       expected: char) -> bool {
        if is_at_end { return false; }
        if self.source.chars().nth(*current) != Some(expected) { return false; }

        *current += 1;
        true
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

#[derive(Debug, Clone)]
pub struct Token { 
    pub token_type: TokenType,
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

