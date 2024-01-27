use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

// impl Program {
//     pub fn token_literal(&self) -> String {
//         if self.statements.len() > 0 {
//             return self.statements[0].token_literal();
//         } else {
//             return String::from("");
//         }
//     }
// }
