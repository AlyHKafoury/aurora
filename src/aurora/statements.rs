use crate::aurora::expressions::Expression;
use crate::aurora::token::Token;

use super::environment::{Memory, Environment};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Class {
        name: Token,
        superclass: Expression,
        methods: Vec<Statement>,
    },
    Expression {
        expression: Expression,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },
    Print {
        expression: Expression,
    },
    Return {
        keyword: Token,
        value: Expression,
    },
    Variable {
        name: Token,
        init: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

impl Statement {
    pub fn evaluate(&mut self, env: &mut Environment) {
        match &*self {
            Statement::Print { expression: expr } => {
                println!("{:?}", expr.evaluate(env))
            }
            Statement::Expression { expression: expr } => {
                println!("{:?}", expr.evaluate(env))
            }
            Statement::Variable {
                name: n,
                init,
            } => {
                let value = match init.clone() {
                    Some(expr) => expr.evaluate(env),
                    None => super::expressions::Object::NilObject,
                };

                env.define(n.clone(), value);
            }
            Statement::Block { statements } => {
                env.stackpush();
                for stmnt in statements.clone().iter_mut() {
                    stmnt.evaluate(env);
                }
                env.stackpop();
            }
            _ => panic!("Invalid Statement"),
        }
    }
}
