use crate::aurora::expressions::Expression;
use crate::aurora::token::Token;

use super::{environment::Environment, expressions::Object};

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
        body: Box<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expression: Expression,
    },
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
    Variable {
        name: Token,
        init: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        init: Box<Option<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
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
                expr.evaluate(env);
            }
            Statement::Variable { name: n, init } => {
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
                    if env.is_set_return() {
                        break;
                    }
                }
                env.stackpop();
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => match condition.clone().evaluate(env) {
                Object::BoolObject(x) => match x {
                    true => {
                        then_branch.clone().evaluate(env);
                    }
                    false => match else_branch {
                        Some(b) => b.clone().evaluate(env),
                        None => (),
                    },
                },
                _ => panic!("Condition should be of type bool {:?}", condition),
            },
            Statement::While { condition, body } => {
                let vals = vec![Object::BoolObject(false), Object::NilObject];
                while !vals.contains(&condition.clone().evaluate(env)) {
                    body.clone().evaluate(env);
                }
            }
            Statement::For {
                init,
                condition,
                increment,
                body,
            } => {
                match *init.clone() {
                    Some(stmnt) => stmnt.clone().evaluate(env),
                    None => (),
                }

                match condition {
                    Some(expr) => {
                        let vals = vec![Object::BoolObject(false), Object::NilObject];
                        while !vals.contains(&expr.clone().evaluate(env)) {
                            body.clone().evaluate(env);
                            match increment {
                                Some(expr) => {
                                    expr.clone().evaluate(env);
                                }
                                None => (),
                            }
                        }
                    }
                    None => {
                        body.clone().evaluate(env);
                        match increment {
                            Some(expr) => {
                                expr.clone().evaluate(env);
                            }
                            None => (),
                        }
                    }
                }
            }
            Statement::Function { name, params, body } => {
                env.define(
                    name.clone(),
                    Object::FunctionObject {
                        name: name.clone(),
                        parameters: params.clone(),
                        body: body.clone(),
                    },
                );
            }
            Statement::Return { keyword, value } => {
                match value {
                    Some(expr) => {
                        let object_value = expr.evaluate(env);
                        env.set_return(object_value);},
                    None => env.set_return(Object::NilObject)
                }
            }
            _ => panic!("Invalid Statement"),
        }
    }
}
