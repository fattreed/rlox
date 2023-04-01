#[cfg(test)]
mod test {
    use std::fs;
    use crate::scanner::{Scanner, TokenType, Token, Literal};

    #[test]
    fn test_scan_token_type() {
        let source = fs::read_to_string("test.lox");
        if let Ok(s) = source {
            let scanner = Scanner::new(s);
            let expected_tokens = vec![
                TokenType::LEFT_PAREN,
                TokenType::RIGHT_PAREN,
                TokenType::LEFT_BRACE,
                TokenType::RIGHT_BRACE,
                TokenType::COMMA,
                TokenType::DOT,
                TokenType::MINUS,
                TokenType::PLUS,
                TokenType::SEMICOLON,
                TokenType::STAR,
                TokenType::BANG,
                TokenType::BANG_EQUAL,
                TokenType::EQUAL,
                TokenType::EQUAL_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL,
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::STRING,
                TokenType::NUMBER,
                TokenType::IDENTIFIER,
                TokenType::CLASS,
                TokenType::FALSE,
                TokenType::EOF,
            ];
           

            let tokens = scanner.scan_tokens();
            for (i, token) in tokens.iter().enumerate() {
                println!("{:?}", token.token_type);
                assert_eq!(token.token_type, expected_tokens[i]);
            }

        }
    }

    #[test]
    fn test_scan_string() {
        let source = "\"this is a test string\"".to_string();
        let scanner = Scanner::new(source);
        let _expected_token = Token {
            token_type: TokenType::STRING,
            lexeme: "this is a test string".to_string(),
            literal: Literal::String("this is a test string".to_string()),
            line: 1,
        };

        let token = scanner.scan_tokens()[0].clone();
        assert_eq!(token.token_type, TokenType::STRING);
        assert_eq!(token.lexeme, "this is a test string");
        assert_eq!(token.literal, Literal::String("this is a test string".to_string()));
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_scan_num() {
        let source = "420 69 4 2 0 6 9".to_string();
        let scanner = Scanner::new(source);
        let _expected_token = Token {
            token_type: TokenType::NUMBER,
            lexeme: "420".to_string(),
            literal: Literal::Number(420.0),
            line: 1,
        };

        let token = scanner.scan_tokens()[0].clone();
        assert_eq!(token.token_type, TokenType::NUMBER);
        assert_eq!(token.lexeme, "420".to_string());
        assert_eq!(token.literal, Literal::Number(420.0));
        assert_eq!(token.line, 1);

    }

    #[test]
    fn test_scan_ids() {
        let source = "not notReserved words".to_string();
        let scanner = Scanner::new(source);
        let _expected_token = Token {
            token_type: TokenType::IDENTIFIER,
            lexeme: "notReserved".to_string(),
            literal: Literal::None,
            line: 1,
        }; 
        
        let token = scanner.scan_tokens()[1].clone();
        assert_eq!(token.token_type, TokenType::IDENTIFIER);
        assert_eq!(token.lexeme, "notReserved".to_string());
        assert_eq!(token.literal, Literal::None);
        assert_eq!(token.line, 1);
    }
    
    #[test]
    fn test_scan_reserved_words() {
        let source = "not true notReserved and class".to_string();
        let scanner = Scanner::new(source);
        let _expected_token_1 = Token {
            token_type: TokenType::TRUE,
            lexeme: "true".to_string(),
            literal: Literal::None,
            line: 1,
        }; 
        let tokens = scanner.scan_tokens();
        let token_1 = tokens[1].clone();
        assert_eq!(token_1.token_type, TokenType::TRUE);
        assert_eq!(token_1.lexeme, "true".to_string());
        assert_eq!(token_1.literal, Literal::None);
        assert_eq!(token_1.line, 1);

        let _expected_token_2 = Token {
            token_type: TokenType::AND,
            lexeme: "string".to_string(),
            literal: Literal::None,
            line: 1,
        }; 
        
        let token_2 = scanner.scan_tokens()[3].clone();
        assert_eq!(token_2.token_type, TokenType::AND);
        assert_eq!(token_2.lexeme, "and".to_string());
        assert_eq!(token_2.literal, Literal::None);
        assert_eq!(token_2.line, 1);

        let _expected_token_3 = Token {
            token_type: TokenType::CLASS,
            lexeme: "class".to_string(),
            literal: Literal::None,
            line: 1,
        }; 
        
        let token_3 = scanner.scan_tokens()[4].clone();
        assert_eq!(token_3.token_type, TokenType::CLASS);
        assert_eq!(token_3.lexeme, "class".to_string());
        assert_eq!(token_3.literal, Literal::None);
        assert_eq!(token_3.line, 1);

    }
}

