#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(usize),

    PLUS,
    ASSIGN,

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
    kind: TokenType,
    literal: String,
}

impl Token {
    pub fn new_token(token_type: TokenType, ch: char) -> Token {
        return Token {
            kind: token_type,
            literal: ch.to_string(),
        };
    }
}
