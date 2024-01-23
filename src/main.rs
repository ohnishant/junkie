mod lexer;
mod repl;
mod utils;

pub mod ast;
pub mod parser;
pub mod token;

use repl::start;

fn main() {
    start();
}
