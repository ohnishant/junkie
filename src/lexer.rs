#![allow(dead_code)]

use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// Returns a new lexer object with position 0 for input String
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            current_char: Some('\0'),
            position: 0,
        }
    }

    fn get_current_char(&mut self) -> Option<char> {
        self.current_char = self.input.chars().nth(self.position);
        self.current_char
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_whitespace(&self, ch: char) -> bool {
        ch.is_whitespace() || ch == '\n' || ch == '\r'
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.get_current_char() {
            if !self.is_whitespace(ch) {
                break;
            } 
            self.advance();
        }
    }

    /// Returns the next character at the next position of Lexer
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }
    
    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        if let Some(ch) = self.get_current_char() {
            match ch {
                '=' => tok = Token::new_token(TokenType::ASSIGN, ch.to_string()),
                ';' => tok = Token::new_token(TokenType::SEMICOLON, ch.to_string()),
                '(' => tok = Token::new_token(TokenType::LPAREN, ch.to_string()),
                ')' => tok = Token::new_token(TokenType::RPAREN, ch.to_string()),
                ',' => tok = Token::new_token(TokenType::COMMA, ch.to_string()),
                '+' => tok = Token::new_token(TokenType::PLUS, ch.to_string()),
                '{' => tok = Token::new_token(TokenType::LBRACE, ch.to_string()),
                '}' => tok = Token::new_token(TokenType::RBRACE, ch.to_string()),
                // '\0' => tok = Token::new_token(TokenType::EOF,"".to_string()),
                _ => tok = Token::new_token(TokenType::ILLEGAL, ch.to_string()),
            }
        }
        else {
            tok = Token::new_token(TokenType::EOF, "".to_string());
        }
        self.advance();
        return tok;
    }
}

#[test]
fn test_next_token() {
    let input = "=+(){}";
    let mut lexer = Lexer::new(input.to_string());
    
    let tests = [
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::PLUS, "+".to_string()),
        Token::new_token(TokenType::LPAREN, "(".to_string()),
        Token::new_token(TokenType::RPAREN, ")".to_string()),
        Token::new_token(TokenType::LBRACE, "{".to_string()),
        Token::new_token(TokenType::RBRACE, "}".to_string()),
        Token::new_token(TokenType::EOF, "".to_string()),
    ];

    for tt in tests.iter() {
        let tok = lexer.next_token();
        assert_eq!(tok.kind, tt.kind);
        assert_eq!(tok.literal, tt.literal);
    }
}
