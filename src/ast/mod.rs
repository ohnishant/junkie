use crate::token;

pub trait Node {
    fn token_literal(&self) -> Option<&String>;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

// #[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> Option<&String> {
        return self.expressions.get(0)?.token_literal();
    }
}

// ---------- Identifier --------

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal);
    }
}

// ---------- Let Statement --------

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: dyn Expression,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> Option<&String> {
        return Some(&self.token.literal);
    }
}
