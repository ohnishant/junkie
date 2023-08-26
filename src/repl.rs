use std::io;

use crate::lexer::Lexer;
use crate::token::TokenType;

pub fn start() {
    loop {
        print!(">> ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let mut lexer = Lexer::new(input);
        let mut token = lexer.next_token();

        while token.kind != TokenType::EOF {
            println!("{:?}", token);
            token = lexer.next_token();
        }
    }
}
