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
            let (value, token_type) = self.scan_token(&mut current, start, &mut line, is_at_end);

            match token_type {
                Some(t) => {
                    match value.clone() {
                        Literal::String(s) => {
                            tokens.push(self.create_token_literal(t, s, value, line))
                        }
                        Literal::Number(n) => {
                            tokens.push(self.create_token_literal(t, n.to_string(), value, line))
                        }
                        Literal::None => tokens.push(self.create_token(t, "".to_string(), line)),
                    }
                }
                _ => (),
            }
        }
        tokens.push(self.create_token_literal(TokenType::EOF, 
                                              "".to_string(), 
                                              Literal::None, 
                                              line));
        tokens
    }

    fn create_token(&self, 
                    token_type: TokenType, 
                    text: String, 
                    line: usize) -> Token {
        self.create_token_literal(token_type, text, Literal::None, line)
    }

    fn create_token_literal(&self, 
                            token_type: TokenType, 
                            text: String,
                            literal: Literal,
                            line: usize) -> Token {
        Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line: line,
        } 
    }

    fn scan_token(&self, 
                  current: &mut usize,
                  start: usize,
                  line: &mut usize, 
                  is_at_end: bool) -> (Literal, Option<TokenType>) {
        let c = self.advance(current);
        match c {
            Some('(') => (Literal::None, Some(TokenType::LEFT_PAREN)),
            Some(')') => (Literal::None, Some(TokenType::RIGHT_PAREN)),
            Some('{') => (Literal::None, Some(TokenType::LEFT_BRACE)),
            Some('}') => (Literal::None, Some(TokenType::RIGHT_BRACE)),
            Some(',') => (Literal::None, Some(TokenType::COMMA)),
            Some('.') => (Literal::None, Some(TokenType::DOT)),
            Some('-') => (Literal::None, Some(TokenType::MINUS)),
            Some('+') => (Literal::None, Some(TokenType::PLUS)),
            Some(';') => (Literal::None, Some(TokenType::SEMICOLON)),
            Some('*') => (Literal::None, Some(TokenType::STAR)),
            Some('!') => {
                if self.check_next_char(is_at_end, current, '=') {
                    (Literal::None, Some(TokenType::BANG_EQUAL))
                } else {
                    (Literal::None, Some(TokenType::BANG))
                }
            }
            Some('=') => {
                if self.check_next_char(is_at_end, current, '=') {
                    (Literal::None, Some(TokenType::EQUAL_EQUAL))
                } else {
                    (Literal::None, Some(TokenType::EQUAL))
                }
            }
            Some('<') => {
                if self.check_next_char(is_at_end, current, '=') {
                    (Literal::None, Some(TokenType::LESS_EQUAL))
                } else {
                    (Literal::None, Some(TokenType::LESS))
                }
            }
            Some('>') => {
                if self.check_next_char(is_at_end, current, '=') {
                    (Literal::None, Some(TokenType::GREATER_EQUAL))
                } else {
                    (Literal::None, Some(TokenType::GREATER))
                }
            }
            Some('/') => {
                if self.check_next_char(is_at_end, current, '/') {
                    while self.peek(*current, is_at_end) != Some('\n') && !is_at_end {
                        _ = self.advance(current);
                    }
                    (Literal::None, None)
                } else {
                    (Literal::None, Some(TokenType::SLASH))
                }
            }
            Some('"') => self.string(current, start, is_at_end, line),
            Some(' ') | Some('\r') | Some('\t') => (Literal::None, None),
            Some('\n') => { *line += 1; (Literal::None, None) }
            _ => {
                if Self::is_digit(c) {
                    self.number(current, is_at_end, start)
                } else {
                    eprint!("unexpected token: {}", 0);
                    (Literal::None, None)
                }
            }
        }
    }

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some(digit) => digit >= '0' && digit <= '9',
            None => false,
        }
        
    }

    fn number(&self, current: &mut usize, is_at_end: bool, start: usize) -> (Literal, Option<TokenType>) {
        while Self::is_digit(self.peek(*current, is_at_end)) {
            self.advance(current);
        }

        if self.peek(*current, is_at_end) == Some('.') && Self::is_digit(self.peek_next(*current)) {
            self.advance(current);
            while Self::is_digit(self.peek(*current, is_at_end)) {
                self.advance(current);
            }
        }

        let num = &self.source[start..*current];
        let num_val = num.parse::<f64>().unwrap();
        println!("{:?}", num);
        println!("{:?}", num_val);
        (Literal::Number(num_val), Some(TokenType::NUMBER))
    }

    fn string(&self, 
              current: &mut usize, 
              start: usize, 
              is_at_end: bool, 
              line: &mut usize) -> (Literal, Option<TokenType>) {
        while self.peek(*current, is_at_end) != Some('"') && !is_at_end {
            if self.peek(*current, is_at_end) == Some('\n') { *line += 1 }
            self.advance(current);
        }

        if is_at_end {
            eprint!("error line {} unterminated string", line);
            return (Literal::None, None);
        }

        self.advance(current);

        let value = &self.source[start+1..*current-1];
        (Literal::String(value.to_string()), Some(TokenType::STRING))
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

    fn peek_next(&self, current: usize) -> Option<char> {
        if current + 1 >= self.source.chars().count() {
            return Some('\0');
        }
        self.source.chars().nth(current + 1)
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

