use crate::aurora::token::{Token, TokenType};
use std::mem;

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Object {
    StringObject(String),
    NumberObject(f64),
    BoolObject(bool),
    NilObject(Option<usize>)
}

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Expression {
    Assign{name: Token, value: Box<Expression>},
    Binary{left: Box<Expression>, operator: Token, right: Box<Expression>},
    Call{callee: Box<Expression>, paren: Token, arguments: Vec<Expression>},
    Get{object: Box<Expression>, name: Token},
    Grouping{expression: Box<Expression>},
    Literal{value: Object},
    Logical{left: Box<Expression>, operator: Token, right: Box<Expression>},
    Set{object: Box<Expression>, name: Token, value: Box<Expression>},
    Super{keyword: Token, method: Token},
    This{keyword: Token},
    Unary{operator: Token, right: Box<Expression>},
    Variable{name: Token},
}

impl Expression {
    pub fn evaluate(&self) -> Object {
        match self {
            Expression::Binary{left: l, operator: op, right: r} => {
                let left_value = l.evaluate();
                let right_value = r.evaluate();
                if mem::discriminant(&left_value) != mem::discriminant(&right_value) {
                    panic!("Left object {:?} not the same type as right object {:?}", left_value, right_value);
                }
                match op.tokentype {
                    TokenType::EqualEqual => {
                        return Object::BoolObject(left_value == right_value);
                    }
                    TokenType::BangEqual => {
                        return Object::BoolObject(left_value != right_value);
                    }
                    TokenType::GreaterEqual => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::BoolObject(x >= y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Greater => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::BoolObject(x > y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::LessEqual => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::BoolObject(x <= y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Less => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::BoolObject(x < y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Plus => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::NumberObject(x + y);
                        }
                        else if let (Object::StringObject(x), Object::StringObject(y)) = (&left_value,&right_value) {
                            let mut temp_string = x.clone();
                            temp_string.push_str(&y);
                            return Object::StringObject(temp_string);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Minus => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::NumberObject(x - y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Slash => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::NumberObject(x / y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    TokenType::Star => {
                        if let (Object::NumberObject(x), Object::NumberObject(y)) = (&left_value,&right_value) {
                            return Object::NumberObject(x * y);
                        }else {
                            panic!("Operator {} is not valid for values {:?} {:?}", op ,left_value, right_value);
                        }
                    }
                    _ => {panic!("Invalid Operator Type {}", op)}
                }
            }
            Expression::Unary { operator: op, right: r } => {
                let right_value = r.evaluate();

                match op.tokentype {
                    TokenType::Bang => {
                        if let Object::BoolObject(x) = right_value {
                            return Object::BoolObject(!x);
                        }else {
                            panic!("Operator {} is not valid for values {:?}", op , right_value);
                        }
                    }
                    TokenType::Minus => {
                        if let Object::NumberObject(x) = right_value {
                            return Object::NumberObject(-x);
                        }else {
                            panic!("Operator {} is not valid for values {:?}", op , right_value);
                        }
                    }
                    _ => {panic!("Invalid Operator Type {}", op)}   
                }
            }
            Expression::Literal { value:v } => {
                return (*v).clone();
            }
            Expression::Grouping { expression: e } => {
                return e.evaluate();
            }
            _ => {panic!("No implementation")}
        }
    }
}