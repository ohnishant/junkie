#![allow(dead_code)]

use crate::ast;
use crate::lexer;
use crate::token;

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
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<&ast::Program> {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Statement};

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
            match &program.statements[i] {
                Statement::Let(let_statement) => {
                    assert_eq!(let_statement.name.name, String::from(*exp_ident));
                    match &let_statement.value {
                        Expression::Literal(val) => {
                            assert_eq!(val.value, String::from(*exp_value))
                        }
                        _ => panic!("Expected a literal value at index {}", i),
                    }
                }
            }
        }
    }
}
