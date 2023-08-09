#![allow(dead_code)]

use crate::token::Token;
use crate::token::TokenType;
use crate::utils::{is_digit, is_letter};

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
        self.get_current_char();
    }
    fn retreat(&mut self) {
        self.position -= 1;
        self.get_current_char();
    }

    fn is_whitespace(&self, ch: char) -> bool {
        return ch.is_whitespace();
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

    pub fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.get_current_char() {
            if !is_letter(ch) {
                break;
            }
            identifier.push(ch);
            self.advance();
        }
        self.retreat();
        return identifier;
    }

    pub fn read_number(&mut self) -> String {
        let mut number: String = String::new();

        while let Some(ch) = self.get_current_char() {
            if !is_digit(ch) {
                break;
            }
            number.push(ch);
            self.advance();
        }
        self.retreat();
        return number;
    }

    pub fn next_token(&mut self) -> Token {
        let tok_type: TokenType;
        let tok_literal: String;

        if let Some(ch) = self.get_current_char() {
            match ch {
                '=' => {
                    tok_type = TokenType::ASSIGN;
                    tok_literal = ch.to_string();
                }
                ';' => {
                    tok_type = TokenType::SEMICOLON;
                    tok_literal = ch.to_string();
                }
                '(' => {
                    tok_type = TokenType::LPAREN;
                    tok_literal = ch.to_string();
                }
                ')' => {
                    tok_type = TokenType::RPAREN;
                    tok_literal = ch.to_string();
                }
                ',' => {
                    tok_type = TokenType::COMMA;
                    tok_literal = ch.to_string();
                }
                '+' => {
                    tok_type = TokenType::PLUS;
                    tok_literal = ch.to_string();
                }
                '{' => {
                    tok_type = TokenType::LBRACE;
                    tok_literal = ch.to_string();
                }
                '}' => {
                    tok_type = TokenType::RBRACE;
                    tok_literal = ch.to_string();
                }
                // '\0' => {
                //     tok_type = TokenType::EOF;
                //     tok_literal = "".to_string();
                // }
                _ => {
                    if is_letter(ch) {
                        tok_literal = self.read_identifier();
                        tok_type = match tok_literal.as_str() {
                            "let" => TokenType::LET,
                            "fn" => TokenType::FUNCTION,
                            _ => TokenType::IDENT(tok_literal.clone()),
                        }
                    } else if is_digit(ch) {
                        tok_literal = self.read_number();
                        tok_type = TokenType::INT(tok_literal.parse().unwrap());
                    } else {
                        tok_type = TokenType::ILLEGAL;
                        tok_literal = ch.to_string();
                    }
                }
            }
        } else {
            tok_type = TokenType::EOF;
            tok_literal = "".to_string();
        }

        self.advance();
        self.skip_whitespace();

        println!("{}", tok_literal);

        return Token {
            kind: tok_type,
            literal: tok_literal,
        };
    }
}

#[test]
fn test_next_token_second() {
    let input = "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    }

    let result = add(five, ten);
    ";

    let mut lexer = Lexer::new(input.to_string());

    let tests = [
        Token::new_token(TokenType::LET, "let".to_string()),
        Token::new_token(TokenType::IDENT("five".to_string()), "five".to_string()),
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::INT(5), "5".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        Token::new_token(TokenType::LET, "let".to_string()),
        Token::new_token(TokenType::IDENT("ten".to_string()), "ten".to_string()),
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::INT(10), "10".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        Token::new_token(TokenType::LET, "let".to_string()),
        Token::new_token(TokenType::IDENT("add".to_string()), "add".to_string()),
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::FUNCTION, "fn".to_string()),
        Token::new_token(TokenType::LPAREN, "(".to_string()),
        Token::new_token(TokenType::IDENT("x".to_string()), "x".to_string()),
        Token::new_token(TokenType::COMMA, ",".to_string()),
        Token::new_token(TokenType::IDENT("y".to_string()), "y".to_string()),
        Token::new_token(TokenType::RPAREN, ")".to_string()),
        Token::new_token(TokenType::LBRACE, "{".to_string()),
        Token::new_token(TokenType::IDENT("x".to_string()), "x".to_string()),
        Token::new_token(TokenType::PLUS, "+".to_string()),
        Token::new_token(TokenType::IDENT("y".to_string()), "y".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        Token::new_token(TokenType::RBRACE, "}".to_string()),
        Token::new_token(TokenType::LET, "let".to_string()),
        Token::new_token(TokenType::IDENT("result".to_string()), "result".to_string()),
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::IDENT("add".to_string()), "add".to_string()),
        Token::new_token(TokenType::LPAREN, "(".to_string()),
        Token::new_token(TokenType::IDENT("five".to_string()), "five".to_string()),
        Token::new_token(TokenType::COMMA, ",".to_string()),
        Token::new_token(TokenType::IDENT("ten".to_string()), "ten".to_string()),
        Token::new_token(TokenType::RPAREN, ")".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        Token::new_token(TokenType::EOF, "".to_string()),
    ];

    for tt in tests.iter() {
        let tok = lexer.next_token();
        assert_eq!(tok.kind, tt.kind);
        assert_eq!(tok.literal, tt.literal);
    }
}

#[test]
fn test_next_token_first() {
    let input = "=+(){},;";
    // let input = "=+aa(){},;";
    let mut lexer = Lexer::new(input.to_string());

    let tests = [
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::PLUS, "+".to_string()),
        // Token::new_token(TokenType::ILLEGAL, "a".to_string()),
        // Token::new_token(TokenType::ILLEGAL, "a".to_string()),
        Token::new_token(TokenType::LPAREN, "(".to_string()),
        Token::new_token(TokenType::RPAREN, ")".to_string()),
        Token::new_token(TokenType::LBRACE, "{".to_string()),
        Token::new_token(TokenType::RBRACE, "}".to_string()),
        Token::new_token(TokenType::COMMA, ",".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        Token::new_token(TokenType::EOF, "".to_string()),
    ];

    for tt in tests.iter() {
        let tok = lexer.next_token();
        assert_eq!(tok.kind, tt.kind);
        assert_eq!(tok.literal, tt.literal);
    }
}
