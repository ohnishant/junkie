#![allow(dead_code)]

use crate::token;
use crate::token::Token;
use crate::token::TokenType;


#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
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
    
    pub fn read_identifier(&mut self) -> String {
        let mut idntifier = String::new();

        while let Some(ch) = self.get_current_char() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                break;
            }
            idntifier.push(ch);
            self.advance();
        }
        return idntifier;
    }

    fn lookup_ident(&mut self,  tok: &mut token::Token) -> token::TokenType {
        match tok.literal.as_str() {
            "fn" => return TokenType::FUNCTION,
            "let" => return TokenType::LET,
            _ => return TokenType::IDENT("thing".to_string()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token;

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
                _ => {
                    if is_letter(ch) {
                        let lit = self.read_identifier();
                        let tok_kind = self.lookup_ident(&mut tok);
                        tok = Token::new_token(tok_kind, lit)
                    }
                    else {
                        tok = Token::new_token(TokenType::ILLEGAL, ch.to_string())
                    }
                },
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
        Token::new_token(TokenType::LPAREN, "(".to_string()),
        Token::new_token(TokenType::SEMICOLON, ";".to_string()),
        
        Token::new_token(TokenType::LET, "let".to_string()),
        Token::new_token(TokenType::IDENT("ten".to_string()), "ten".to_string()),
        Token::new_token(TokenType::ASSIGN, "=".to_string()),
        Token::new_token(TokenType::INT(10), "10".to_string()),
        Token::new_token(TokenType::LPAREN, "(".to_string()),
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


// #[test]
// fn test_next_token() {
//     let input = "=+aa(){}";
//     let mut lexer = Lexer::new(input.to_string());
    
//     let tests = [
//         Token::new_token(TokenType::ASSIGN, "=".to_string()),
//         Token::new_token(TokenType::PLUS, "+".to_string()),
//         Token::new_token(TokenType::ILLEGAL, "a".to_string()),
//         Token::new_token(TokenType::ILLEGAL, "a".to_string()),
//         Token::new_token(TokenType::LPAREN, "(".to_string()),
//         Token::new_token(TokenType::RPAREN, ")".to_string()),
//         Token::new_token(TokenType::LBRACE, "{".to_string()),
//         Token::new_token(TokenType::RBRACE, "}".to_string()),
//         Token::new_token(TokenType::EOF, "".to_string()),
//     ];

//     for tt in tests.iter() {
//         let tok = lexer.next_token();
//         assert_eq!(tok.kind, tt.kind);
//         assert_eq!(tok.literal, tt.literal);
//     }
// }
