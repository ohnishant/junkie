use std::io::{self, stdout, Write};

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start() {
    loop {
        let mut input: String = String::new();
        print!(">> ");
        stdout().flush().expect("Error flushing stdout.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(input);
        let mut token = lexer.next_token();

        while token.kind != TokenType::EOF {
            println!("Type: {:?}  Literal: {}", token.kind, token.literal);
            token = lexer.next_token();
        }
        print!("\n\n")
    }
}
