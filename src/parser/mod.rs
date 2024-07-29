#![allow(dead_code)]

use crate::ast;
use crate::ast::Identifier;
use crate::lexer;
use crate::token;
use crate::token::TokenType;

#[derive(Debug)]
struct Parser {
    lexer: lexer::Lexer,

    current_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        return Parser {
            lexer,
            current_token,
            peek_token,
        };
    }
    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.kind {
            token::TokenType::LET => return self.parse_let_statement(),
            _ => {
                print!(
                    "Parsing error: Unknown token type: {:?}",
                    self.current_token.kind
                );
                return None;
            }
        }
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        // create root node
        let mut program = ast::Program {
            statements: Vec::new(),
        };
        // populate root node with statements
        while self.current_token.kind != token::TokenType::EOF {
            let stms = self.parse_statement();
            if let Some(stm) = stms {
                program.statements.push(stm);
            }
            self.next_token();
        }
        return Some(program);
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        // skip over the let keyworkd since we know this is a let Statement
        self.next_token();

        // find out the name of the identifer that is being declared
        let ident = if let TokenType::IDENT(name) = &self.current_token.kind {
            Identifier {
                name: name.to_string(),
            }
        } else {
            return None;
        };

        // Check if an assignment operator is present
        self.next_token();
        if !(self.current_token.kind == TokenType::ASSIGN) {
            return None;
        }

        //TODO: skipping over the expression for now

        let stmt = ast::LetStatement {
            token: self.current_token.clone(),
            name: ident,
            //TODO: skipping over the expression for now
            value: ast::Expression::Literal(ast::Literal {
                value: "0".to_string(),
            }),
        };

        return Some(ast::Statement::Let(stmt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = lexer::Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_some(), "Parsing error: Program is has no nodes");

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
}
