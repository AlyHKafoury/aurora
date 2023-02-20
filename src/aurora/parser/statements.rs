use crate::aurora::token::Token;
use crate::aurora::parser::expressions::Expression;

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Statement {
    Block{statements: Vec<Statement>},
    Class{name: Token, superclass: Expression, methods: Vec<Statement>},
    Expression{expression: Expression},
    Function{name: Token, params: Vec<Token>, body: Vec<Statement>},
    If{condition: Expression, then_branch: Box<Statement>, else_branch: Box<Statement>},
    Print{expression: Expression},
    Return{keyword: Token, value: Expression},
    Variable{name: Token, init: Option<Expression>},
    While{condition: Expression, body: Box<Statement>},
}

impl Statement {
    pub fn evaluate(&mut self) {
        match &*self {
        Statement::Print { expression: expr } => {
            println!("{:?}", expr.evaluate())
        }
        Statement::Expression { expression: expr } => {
            println!("{:?}", expr.evaluate())
        }
        _ => panic!("Invalid Statement")
        }
    }
}