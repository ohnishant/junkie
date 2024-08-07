#![allow(dead_code)]

use crate::ast;
use crate::ast::Identifier;
use crate::lexer::Lexer;
use crate::token;
use crate::token::TokenType;

#[derive(Debug)]
struct Parser {
    lexer: Lexer,

    current_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        };
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn errors(&self) -> Vec<String> {
        // TODO: do this without cloning
        return self.errors.clone();
    }

    fn peek_error(&mut self, kind: TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            kind, self.peek_token.kind
        );
        self.errors.push(msg);
    }

    fn expect_peek(&mut self, kind: TokenType) -> bool {
        if TokenType::variant_eq(&self.peek_token.kind, &kind) {
            self.next_token();
            return true;
        } else {
            self.peek_error(kind);
            return false;
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        // create root node
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        // populate root node with statements
        while self.current_token.kind != token::TokenType::EOF {
            if let Some(stm) = self.parse_statement() {
                program.statements.push(stm);
            }
            // jumps over the semicolon
            self.next_token();
        }
        return Some(program);
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.kind {
            token::TokenType::LET => return self.parse_let_statement(),
            token::TokenType::RETURN => return self.parse_return_statement(),
            _ => {
                println!(
                    "[WARN]: Parsing error: Unknown token type: {:?}",
                    self.current_token.kind
                );
                return None;
            }
        }
    }

    /// Parses a let statement and returns a Statement node
    /// The current token should be the LET keyword
    ///
    /// Leaves the parser with the current token as the SEMICOLON
    /// make sure to call next_token() after this function
    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let let_token = self.current_token.clone();

        self.expect_peek(TokenType::IDENT("".to_string()));

        let ident = Identifier {
            name: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        let stmt = ast::LetStatement {
            token: let_token,
            name: ident,
            //TODO: skipping over the expression for now
            value: ast::Expression::Literal(ast::Literal {
                value: "0".to_string(),
            }),
        };

        while self.current_token.kind != TokenType::SEMICOLON {
            self.next_token();
        }

        //println!("[INFO]: Current Token: {:?}", self.current_token);
        //println!("[INFO]: Peek Token: {:?}", self.peek_token);

        //println!("[INFO]: Let Statement: {:?}", stmt);
        return Some(ast::Statement::Let(stmt));
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let return_token = self.current_token.clone();

        self.next_token();
        // TODO: skipping over the expression for now
        while !TokenType::variant_eq(&self.current_token.kind, &TokenType::SEMICOLON) {
            self.next_token();
        }

        let stmt = ast::ReturnStatement {
            token: return_token,
            return_value: ast::Expression::Literal(ast::Literal {
                value: "0".to_string(),
            }),
        };

        return Some(ast::Statement::Return(stmt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_some(), "Parsing error: Program is has no nodes");

        assert!(
            parser.errors().is_empty(),
            "Parser has {} errors:\n\t{}",
            parser.errors().len(),
            parser.errors().join("\n\t")
        );

        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "Found {} statements, expected 3",
            program.statements.len()
        );

        let expected_identifiers = vec![("x", "5"), ("y", "10"), ("foobar", "838383")];

        for (i, (exp_ident, exp_value)) in expected_identifiers.iter().enumerate() {
            let stmt = &program.statements[i];

            match stmt {
                ast::Statement::Let(l_stmt) => {
                    assert_eq!(l_stmt.name.name, *exp_ident);
                }
                _ => panic!("Expected LetStatement, got {:?}", stmt),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"return 5;
        return 10;
        return 993322;
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_some(), "Parsing error: Program is has no nodes");

        assert!(
            parser.errors().is_empty(),
            "Parser has {} errors:\n\t{}",
            parser.errors().len(),
            parser.errors().join("\n\t")
        );

        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "Found {} statements, expected 3",
            program.statements.len()
        );

        for (i, statement) in program.statements.iter().enumerate() {
            match statement {
                ast::Statement::Return(stmt) => {
                    assert_eq!(
                        stmt.token.kind,
                        TokenType::RETURN,
                        "Expected Return token, got {:?}",
                        stmt.token.kind
                    );
                }
                _ => {
                    panic!("Expected ReturnStatement, got {:?}", statement);
                }
            }
        }
    }
}
