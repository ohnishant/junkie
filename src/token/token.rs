type TokenType = String; 

#[derive(Debug)]
struct Token {
    Type: TokenType,
    Literal: str,
}

// Literals
const IDENT: String = "IDENT";
const EOF: String = "EOF";

// Delimiters
const SEMICOLON: String = ";";
const COMMA: String = ",";

const LBRACE: String = "{";
const RBRACE: String = "}";
const LPAREN: String = "(";
const RPAREN: String = ")";

// Keywords
const LET: String = "LET";
const FUNCTION: String = "FUNCTION";
