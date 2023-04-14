enum TokenType {
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
