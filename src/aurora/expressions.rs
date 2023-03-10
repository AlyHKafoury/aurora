use crate::aurora::token::{Token, TokenType};
use std::mem;
use chrono::prelude::*;

use super::{
    environment::{Environment, Memory},
    statements::Statement,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FunctionType {
    Function,
    Method,
    Constructor,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InternalFunction {
    Time,
    Clock
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Object {
    StringObject(String),
    NumberObject(f64),
    BoolObject(bool),
    InternalFunction{
        internaltype: InternalFunction,
    },
    FunctionObject {
        name: Token,
        parameters: Vec<Token>,
        body: Box<Statement>,
        captures: Vec<(Token, Object)>,
        functype: FunctionType,
    },
    Class {
        name: Token,
        class_env: Box<Environment>,
    },
    ClassInstance {
        name: Token,
        class: Box<Object>,
        memory: Memory,
    },
    ThisObject,
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
                return v.clone();
            }
            Expression::Grouping { expression: e } => {
                return e.evaluate(env);
            }
            Expression::Assign { name: n, value: v } => {
                let value = v.evaluate(env);
                let injected_v = match value {
                    Object::ClassInstance {
                        name: _,
                        class,
                        memory,
                    } => Object::ClassInstance {
                        name: n.clone(),
                        class,
                        memory,
                    },
                    _ => value,
                };
                env.assign(n.clone(), injected_v.clone());
                return injected_v;
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
            Expression::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = callee.evaluate(env);
                match callee.clone() {
                    Object::FunctionObject {
                        name,
                        parameters,
                        body,
                        captures,
                        functype,
                    } => {
                        if arguments.len() != parameters.len() {
                            panic!("Wrong Number of arguments for function {}", &name);
                        }
                        for capture in captures {
                            env.inject(capture.0, capture.1);
                        }
                        let arguments_values: Vec<Object> =
                            arguments.into_iter().map(|x| x.evaluate(env)).collect();
                        for i in 0..parameters.len() {
                            env.inject(parameters[i].clone(), arguments_values[i].clone());
                        }
                        env.set_in_function(functype);
                        body.evaluate(env);
                        env.clear_class_instance();
                        env.clear_in_function();
                        return env.unset_return();
                    }
                    Object::Class {
                        name: n,
                        mut class_env,
                    } => {
                        let constructor = class_env.get(n.clone());
                        let mut instance_memory = Memory::new();
                        match constructor {
                            Object::FunctionObject {
                                name: constructor_name,
                                parameters,
                                body,
                                captures,
                                functype: _,
                            } => {
                                class_env.define(
                                    n.clone(),
                                    Object::FunctionObject {
                                        name: constructor_name,
                                        parameters,
                                        body,
                                        captures,
                                        functype: FunctionType::Constructor,
                                    },
                                );
                                class_env.set_in_function(FunctionType::Constructor);
                                class_env.stackpush(instance_memory);
                                let init = Expression::Call {
                                    callee: Box::new(Expression::Variable { name: n.clone() }),
                                    paren: paren.clone(),
                                    arguments: arguments.clone(),
                                };
                                init.evaluate(&mut class_env);
                                class_env.clear_in_function();
                                instance_memory = match class_env.stackpop() {
                                    Some(m) => m,
                                    _ => Memory::new(),
                                }
                            }
                            Object::NilObject => (),
                            _ => panic!("Invalid constructor for {:#?}", n),
                        }
                        let instance = Object::ClassInstance {
                            name: n.clone(),
                            class: Box::new(callee.clone()),
                            memory: instance_memory,
                        };
                        return instance;
                    }
                    Object::InternalFunction { internaltype } => {
                        match internaltype {
                            InternalFunction::Time => {
                                return Object::StringObject(Local::now().to_string())
                            }
                            InternalFunction::Clock => {
                                return Object::NumberObject(Local::now().timestamp() as f64)
                            }
                        }
                    }
                    _ => panic!("Object {} not a function at {}", paren.lexeme, paren.line),
                }
            }
            Expression::Get { object, name } => {
                let envname = match &*(*object) {
                    Expression::Variable { name } => name.clone(),
                    Expression::This { keyword } => keyword.clone(),
                    _ => panic!("Must set property on object {:?}", object),
                };
                let instance = object.evaluate(env);
                match instance {
                    Object::ClassInstance {
                        name: _,
                        class,
                        memory,
                    } => {
                        match *class {
                            Object::Class {
                                name: _,
                                mut class_env,
                            } => {
                                class_env.stackpush(memory);
                                let value = class_env.get(name.clone());
                                class_env.stackpop();
                                env.set_class_instance(envname);
                                return value;
                            }
                            _ => panic!("instance parent is not a class {:?}", &class),
                        };
                    }
                    _ => panic!(
                        "cannot call property {} on non-instance object {:?}",
                        name.clone(),
                        object
                    ),
                }
            }
            Expression::Set {
                object,
                name,
                value,
            } => {
                let instance = object.evaluate(env);
                let set_value = value.evaluate(env);
                match instance.clone() {
                    Object::ClassInstance {
                        name: n,
                        class: c,
                        mut memory,
                    } => {
                        (&mut memory).define(name.clone(), set_value);
                        let instance = Object::ClassInstance {
                            name: n,
                            class: c,
                            memory,
                        };
                        let envname = match &*(*object) {
                            Expression::Variable { name } => name.clone(),
                            Expression::This { keyword:_ } => match env.is_class_instance() {
                                Some(x) => x,
                                None => panic!("Invalid class instance {:#?}", instance),
                            },
                            _ => panic!("Must set property on object {:?}", object),
                        };
                        env.assign(envname, instance.clone());
                        if !env.is_in_method() {
                            env.clear_class_instance();
                        }
                    }
                    Object::ThisObject => env.assign_instance(name.clone(), set_value),
                    _ => panic!(
                        "cannot call property {} on non-instance object {:?}",
                        name.clone(),
                        object
                    ),
                }
                return Object::NilObject;
            }
            Expression::This { keyword } => {
                if env.is_in_constructor() {
                    return Object::ThisObject;
                }
                match (env.is_class_instance(), env.is_in_method()) {
                    (Some(x), true) => return env.get(x),
                    _ => panic!(
                        "Invalid use of this outside of class or outside of method at {}",
                        keyword
                    ),
                }
            }
            Expression::Super { keyword, method } => {
                if env.is_in_constructor() {
                    return env.get(method.clone());
                }
                match (env.is_class_instance(), env.is_in_method()) {
                    (Some(x), true) => match env.get(x.clone()) {
                        Object::ClassInstance { name:_, class, memory:_ } => {
                            match *class {
                               Object::Class { name:n, class_env } => {
                                // panic!("TEst {:#?}", env.get(n.clone()));
                                return class_env.get_from_parent(method.clone())
                               },
                               _ => panic!(
                                "Invalid use of Super outside of class or outside of method at {}",
                                keyword
                            ),
                            }
                        },
                        _ => panic!(
                            "Invalid use of Super outside of class or outside of method at {}",
                            keyword
                        ),
                    },
                    _ => panic!(
                        "Invalid use of Super outside of class or outside of method at {}",
                        keyword
                    ),
                }                
            }
        }
    }

    pub fn resolve(&self, captures: &mut Vec<(Token, Object)>, env: &Environment) {
        match self {
            Expression::Assign { name, value } => {
                value.resolve(captures, env);
                match env.need_to_capture(name.clone()) {
                    true => captures.push((name.clone(), env.get(name.clone()))),
                    false => (),
                }
            }
            Expression::Binary {
                left,
                operator: _,
                right,
            } => {
                left.resolve(captures, env);
                right.resolve(captures, env);
            }
            Expression::Call {
                callee,
                paren: _,
                arguments,
            } => {
                callee.resolve(captures, env);
                for v in arguments {
                    v.resolve(captures, env);
                }
            }
            Expression::Get { object, name: _ } => {
                object.resolve(captures, env);
            }
            Expression::Grouping { expression } => expression.resolve(captures, env),
            Expression::Literal { value: _ } => (),
            Expression::Logical {
                left,
                operator: _,
                right,
            } => {
                left.resolve(captures, env);
                right.resolve(captures, env);
            }
            Expression::Set {
                object,
                name: _,
                value,
            } => {
                object.resolve(captures, env);
                value.resolve(captures, env);
            }
            Expression::Super { keyword:_, method:_ } => (),
            Expression::This { keyword: _ } => (),
            Expression::Unary { operator: _, right } => right.resolve(captures, env),
            Expression::Variable { name } => match env.need_to_capture(name.clone()) {
                true => captures.push((name.clone(), env.get(name.clone()))),
                false => (),
            },
        }
    }
}
