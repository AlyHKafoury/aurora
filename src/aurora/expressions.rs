use crate::aurora::token::{Token, TokenType};
use std::mem;

use super::{environment::Environment, statements::Statement};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Object {
    StringObject(String),
    NumberObject(f64),
    BoolObject(bool),
    FunctionObject{
        name: Token,
        parameters: Vec<Token>,
        body: Box<Statement>,
        captures: Vec<(Token, Object)>,

    },
    NilObject,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expression {
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: Token,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },
}

impl Expression {
    pub fn evaluate(&self, env: &mut Environment) -> Object {
        match self {
            Expression::Binary {
                left: l,
                operator: op,
                right: r,
            } => {
                let left_value = l.evaluate(env);
                let right_value = r.evaluate(env);
                if mem::discriminant(&left_value) != mem::discriminant(&right_value) {
                    panic!(
                        "Left object {:?} not the same type as right object {:?}",
                        left_value, right_value
                    );
                }
                match op.tokentype {
                    TokenType::EqualEqual => {
                        return Object::BoolObject(left_value == right_value);
                    }
                    TokenType::BangEqual => {
                        return Object::BoolObject(left_value != right_value);
                    }
                    TokenType::GreaterEqual => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::BoolObject(x >= y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Greater => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::BoolObject(x > y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::LessEqual => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::BoolObject(x <= y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Less => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::BoolObject(x < y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Plus => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::NumberObject(x + y);
                        } else if let (Object::StringObject(x), Object::StringObject(y)) =
                            (&left_value, &right_value)
                        {
                            let mut temp_string = x.clone();
                            temp_string.push_str(&y);
                            return Object::StringObject(temp_string);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Minus => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::NumberObject(x - y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Slash => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::NumberObject(x / y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    TokenType::Star => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) =
                            (&left_value, &right_value)
                        {
                            return Object::NumberObject(x * y);
                        } else {
                            panic!(
                                "Operator {} is not valid for values {:?} {:?}",
                                op, left_value, right_value
                            );
                        }
                    }
                    _ => {
                        panic!("Invalid Operator Type {}", op)
                    }
                }
            }
            Expression::Unary {
                operator: op,
                right: r,
            } => {
                let right_value = r.evaluate(env);

                match op.tokentype {
                    TokenType::Bang => {
                        if let Object::BoolObject(x) = right_value {
                            return Object::BoolObject(!x);
                        } else {
                            panic!("Operator {} is not valid for values {:?}", op, right_value);
                        }
                    }
                    TokenType::Minus => {
                        if let Object::NumberObject(x) = right_value {
                            return Object::NumberObject(-x);
                        } else {
                            panic!("Operator {} is not valid for values {:?}", op, right_value);
                        }
                    }
                    _ => {
                        panic!("Invalid Operator Type {}", op)
                    }
                }
            }
            Expression::Variable { name: n } => {
                return env.get(n.clone());
            }
            Expression::Literal { value: v } => {
                return (*v).clone();
            }
            Expression::Grouping { expression: e } => {
                return e.evaluate(env);
            }
            Expression::Assign { name: n, value: v } => {
                let value = v.evaluate(env);

                env.assign(n.clone(), value.clone());
                return value;
            }
            Expression::Logical {
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env);
                match left {
                    Object::BoolObject(false) | Object::NilObject => {
                        let right = right.evaluate(env);
                        match operator.tokentype {
                            TokenType::And => return Object::BoolObject(false),
                            TokenType::Or => match right {
                                Object::BoolObject(false) | Object::NilObject => {
                                    return Object::BoolObject(false)
                                }
                                _ => return right,
                            },
                            _ => panic!("Wrong Token for logical expression {}", operator),
                        }
                    }
                    _ => match operator.tokentype {
                        TokenType::Or => return left,
                        TokenType::And => match right.evaluate(env) {
                            Object::BoolObject(false) | Object::NilObject => {
                                return Object::BoolObject(false)
                            }
                            _ => return left,
                        },
                        _ => panic!("Wrong Token for logical expression {}", operator),
                    },
                }
            }
            Expression::Call { callee, paren, arguments } => {
                let function = callee.evaluate(env);
                match function {
                    Object::FunctionObject { name, parameters, body , captures} => {
                        if arguments.len() != parameters.len() {
                            panic!("Wrong Number of arguments for function {}", &name);
                        }
                        for capture in captures {
                            env.inject(capture.0, capture.1);
                        }
                        let arguments_values: Vec<Object> = arguments.into_iter().map(|x| x.evaluate(env)).collect();
                        for i in 0..parameters.len() {
                            env.inject(parameters[i].clone(), arguments_values[i].clone());
                        }
                        body.clone().evaluate(env);
                        return env.unset_return();
                    },
                    _ => panic!("Object {} not a function at {}", paren.lexeme ,paren.line)
                }
            }
            _ => panic!("No implementation"),
        }
    }

    pub fn resolve(&self, captures: &mut Vec<(Token, Object)>, env: &Environment) {
        match self {
            Expression::Assign { name, value } => {
                value.resolve(captures, env);
                match env.need_to_capture(name.clone()) {
                    true => captures.push((name.clone(), env.get(name.clone()))),
                    false => ()
                }
            },
            Expression::Binary { left, operator:_, right } => {
                left.resolve(captures, env);
                right.resolve(captures, env);
            },
            Expression::Call { callee, paren:_, arguments } => {
                callee.resolve(captures, env);
                for v in arguments {
                    v.resolve(captures, env);
                }
            },
            Expression::Get { object, name } => todo!(),
            Expression::Grouping { expression } => expression.resolve(captures, env),
            Expression::Literal { value:_ } => (),
            Expression::Logical { left, operator:_, right } => {
                left.resolve(captures, env);
                right.resolve(captures, env);
            },
            Expression::Set { object, name, value } => todo!(),
            Expression::Super { keyword, method } => todo!(),
            Expression::This { keyword } => todo!(),
            Expression::Unary { operator:_, right } => right.resolve(captures, env),
            Expression::Variable { name } => {
                match env.need_to_capture(name.clone()) {
                    true => captures.push((name.clone(), env.get(name.clone()))),
                    false => ()
                }
            },
        }
    } 
}
