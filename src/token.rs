#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(usize),

    PLUS,
    ASSIGN,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LESSTHAN,
    GREATERTHAN,

    EQUAL,
    NOTEQUAL,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn variant_eq(&self, other: &Self) -> bool {
        use TokenType::*;
        match (self, other) {
            (IDENT(_), IDENT(_)) => true,
            (INT(_), INT(_)) => true,
            _ => self == other,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new_token(token_type: TokenType, ch: String) -> Token {
        return Token {
            kind: token_type,
            literal: ch.to_string(),
        };
    }
}
