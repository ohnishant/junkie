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
    
    pub fn next_token(self) {
        let tok: Token;

        if let Some(ch) = self.current_char {
            match ch {
                '=' => tok = Token::new_token(TokenType::ASSIGN, ch),
                _ => tok = Token::new_token(TokenType::ILLEGAL, ch),
            }
        }
    }
}
