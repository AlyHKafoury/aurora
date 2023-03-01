use std::collections::BTreeMap;

use super::{
    expressions::{FunctionType, Object},
    token::Token,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Memory {
    stack: BTreeMap<String, Object>,
    in_function: Option<FunctionType>,
    class_instance: Option<Token>,
}

impl Memory {
    pub fn new() -> Self {
        return Memory {
            stack: BTreeMap::<String, Object>::new(),
            in_function: None,
            class_instance: None,
        };
    }

    pub fn define(&mut self, k: Token, v: Object) {
        self.stack.insert(k.lexeme.clone(), v);
    }

    pub fn get(&self, token: Token) -> Option<Object> {
        match self.stack.get(&token.lexeme) {
            Some(x) => return Some(x.clone()),
            None => None,
        }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), ()> {
        match self.stack.get(&token.lexeme) {
            Some(_) => {
                self.stack.insert(token.lexeme.clone(), value);
                return Ok(());
            }
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Environment {
    stack: Vec<Memory>,
    return_switch: bool,
    return_value: Object,
    injects: Vec<(Token, Object)>,
    in_function: Option<FunctionType>,
    class_instance: Option<Token>,
}

impl Environment {
    pub fn new() -> Self {
        let mut memorystack = Vec::<Memory>::new();
        memorystack.push(Memory::new());
        return Environment {
            stack: memorystack,
            return_switch: false,
            return_value: Object::NilObject,
            injects: Vec::<(Token, Object)>::new(),
            in_function: None,
            class_instance: None,
        };
    }

    pub fn stackpush(&mut self, mut memory: Memory) {
        memory.in_function = self.in_function.clone();
        memory.class_instance = self.class_instance.clone();
        self.class_instance = None;
        self.in_function = None;
        self.stack.push(memory);
        loop {
            match self.injects.pop() {
                Some(i) => self.define(i.0, i.1),
                None => break,
            }
        }
    }

    pub fn debug_last(&self) {
        println!("{:#?}", self.stack.last())
    }

    pub fn stack_temp_push(&mut self) {
        self.stack.push(Memory::new());
    }

    pub fn stack_temp_pop(&mut self) {
        self.stack.pop();
    }

    pub fn inject(&mut self, t: Token, v: Object) {
        self.injects.push((t, v));
    }

    pub fn stackpop(&mut self) -> Option<Memory> {
        self.injects = Vec::<(Token, Object)>::new();
        return self.stack.pop();
    }

    pub fn define(&mut self, k: Token, v: Object) {
        let memorysize = self.stack.len() - 1;
        self.stack[memorysize].define(k, v);
    }

    pub fn get(&self, token: Token) -> Object {
        let mut memorysize = self.stack.len() - 1;
        loop {
            match self.stack[memorysize].get(token.clone()) {
                Some(x) => return x,
                _ => {
                    if memorysize > 0 {
                        memorysize -= 1
                    } else if memorysize == 0 {
                        break;
                    }
                }
            }
        }

        panic!("Undefined variable {}, {:#?}", token.lexeme, self.stack[0]);
    }

    pub fn need_to_capture(&self, token: Token) -> bool {
        let oringal_size = self.stack.len() - 1;
        let mut memorysize = self.stack.len() - 1;

        loop {
            match self.stack[memorysize].get(token.clone()) {
                Some(_) => {
                    if memorysize == oringal_size {
                        return false;
                    } else {
                        return true;
                    }
                }
                _ => {
                    if memorysize > 0 {
                        memorysize -= 1
                    } else if memorysize == 0 {
                        break;
                    }
                }
            }
        }

        panic!("Undefined variable {}, {:?}", token.lexeme, self.stack);
    }

    pub fn assign(&mut self, token: Token, value: Object) {
        let mut memorysize = self.stack.len() - 1;

        loop {
            match self.stack[memorysize].assign(token.clone(), value.clone()) {
                Ok(_) => return,
                _ => {
                    if memorysize > 0 {
                        memorysize -= 1
                    } else if memorysize == 0 {
                        break;
                    }
                }
            }
        }

        panic!("Undefined variable {}", token.lexeme);
    }

    pub fn set_return(&mut self, value: Object) {
        self.return_switch = true;
        self.return_value = value;
    }

    pub fn is_set_return(&self) -> bool {
        return self.return_switch;
    }

    pub fn unset_return(&mut self) -> Object {
        self.return_switch = false;
        let value = self.return_value.clone();
        self.return_value = Object::NilObject;
        return value;
    }

    pub fn set_in_function(&mut self, in_function: Option<FunctionType>) {
        self.in_function = in_function;
    }

    pub fn set_class_instance(&mut self, class_instance: Option<Token>) {
        self.class_instance = class_instance;
    }

    pub fn is_in_function(&self) -> bool{
        match self.stack.last().unwrap().in_function {
            Some(FunctionType::Function) => true,
            _ => false
        }
    }
    pub fn is_in_method(&self) -> bool{
        match self.stack.last().unwrap().in_function {
            Some(FunctionType::Method) => true,
            _ => false
        }
    }
    pub fn is_class_instance(&self) -> Option<Token> {
        self.stack.last().unwrap().class_instance.clone()
    }
    
    pub fn clear_class_instance(&mut self) {
        self.class_instance = None;
    }
}
