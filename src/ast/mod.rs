use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &String {
        todo!()
    }
    fn to_string(&self) -> &String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
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

impl Node for Program {
    // TODO: Implement this. deal with the string lifetimes lol
    fn token_literal(&self) -> &String {
        if self.statements.len() > 0 {
            let statement = &self.statements[0];
            match statement {
                Statement::Let(_) => todo!(),
                Statement::Return(_) => todo!(),
                Statement::Expression(_) => todo!(),
                _ => unimplemented!("[ERROR]: Unimplemented statement type"),
            }
        } else {
            todo!();
        }
    }

    fn to_string(&self) -> &String {
        todo!()
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> &String {
        return &self.token.literal;
    }

    fn to_string(&self) -> &String {
        // FIXME: Implement this
        match &self.value {
            Expression::Identifier(ident) => return &ident.name,
            Expression::Literal(lit) => return &lit.value,
        };
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &String {
        return &self.token.literal;
    }

    fn to_string(&self) -> &String {
        match &self.return_value {
            Expression::Identifier(ident) => return &ident.name,
            Expression::Literal(lit) => return &lit.value,
        };
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &String {
        return &self.token.literal;
    }

    fn to_string(&self) -> &String {
        match &self.expression {
            Expression::Identifier(ident) => return &ident.name,
            Expression::Literal(lit) => return &lit.value,
        };
    }
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
