#[derive(Debug, PartialEq)]
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

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

#[derive(Debug)]
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
