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
                let mut captures = Vec::<(Token, Object)>::new();
                env.stack_temp_push();
                for t in params {
                    env.define(t.clone(), Object::NilObject);                    
                }
                body.resolve(&mut captures, env);
                env.stack_temp_pop();
                env.define(
                    name.clone(),
                    Object::FunctionObject {
                        name: name.clone(),
                        parameters: params.clone(),
                        body: body.clone(),
                        captures: captures,
                    },
                );
            }
            Statement::Return { keyword:_, value } => match value {
                Some(expr) => {
                    let object_value = expr.evaluate(env);
                    env.set_return(object_value);
                }
                None => env.set_return(Object::NilObject),
            },
            _ => panic!("Invalid Statement"),
        }
    }

    pub fn resolve(&self, captures: &mut Vec<(Token, Object)>, env: &mut Environment) {
        match self {
            Statement::Block { statements } => {
                for s in statements {
                    s.resolve(captures, env);
                }
            }
            Statement::Class {
                name,
                superclass,
                methods,
            } => todo!(),
            Statement::Expression { expression } => expression.resolve(captures, env),
            Statement::Function {
                name,
                params,
                body,
            } => {
                env.define(name.clone(), Object::NilObject);
                for t in params {
                    env.define(t.clone(), Object::NilObject);                    
                }
                body.resolve(captures, env);
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                condition.resolve(captures, env);
                then_branch.resolve(captures, env);
                match else_branch {
                    Some(s) => s.resolve(captures, env),
                    None => (),
                }
            }
            Statement::Print { expression } => expression.resolve(captures, env),
            Statement::Return { keyword:_, value } => match value {
                Some(x) => x.resolve(captures, env),
                None => (),
            },
            Statement::Variable { name, init } => {
                env.define(name.clone(), Object::NilObject);
                match init {
                Some(x) => x.resolve(captures, env),
                None => (),
            }},
            Statement::While { condition, body } => {
                condition.resolve(captures, env);
                body.resolve(captures, env);
            },
            Statement::For {
                init,
                condition,
                increment,
                body,
            } => {
                match &*(*init) {
                    Some(s) => s.resolve(captures, env),
                    None => (),
                }
                match condition {
                    Some(x) => x.resolve(captures, env),
                    None => (),
                }
                match increment {
                    Some(x) => x.resolve(captures, env),
                    None => (),
                }
                body.resolve(captures, env);
            },
        }
    }
}
