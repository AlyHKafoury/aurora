use std::collections::HashMap;

use super::{expressions::Object, token::Token};

#[derive(Debug, Clone)]
pub struct Memory(HashMap<String, Object>);

impl Memory {
    pub fn new() -> Self {
        return Memory(HashMap::<String, Object>::new());
    }

    pub fn define(&mut self, k: Token, v: Object) {
        self.0.insert(k.lexeme.clone(), v);
    }

    pub fn get(&self, token: Token) -> Option<Object> {
        match self.0.get(&token.lexeme) {
            Some(x) => return Some(x.clone()),
            None => None,
        }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), ()> {
        match self.0.get(&token.lexeme) {
            Some(_) => {
                self.0.insert(token.lexeme.clone(), value);
                return Ok(());
            }
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<Memory>,
    return_switch: bool,
    return_value: Object,
    injects: Vec<(Token, Object)>,
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
        };
    }

    pub fn stackpush(&mut self) {
        self.stack.push(Memory::new());
        loop {
            match self.injects.pop() {
                Some(i) => self.define(i.0, i.1),
                None => break
            }
        }
    }

    pub fn stack_temp_push(&mut self) {
        self.stack.push(Memory::new()); 
    }

    pub fn stack_temp_pop(&mut self) {
        self.stack.pop();
    }

    pub fn inject(&mut self, t:Token, v: Object) {
        self.injects.push((t, v));
    }

    pub fn stackpop(&mut self) {
        self.stack.pop();
        self.injects = Vec::<(Token, Object)>::new();
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

        panic!("Undefined variable {}", token.lexeme);
    }

    pub fn need_to_capture(&self, token: Token) -> bool {
        let oringal_size = self.stack.len() - 1;
        let mut memorysize = self.stack.len() - 1;

        loop {
            match self.stack[memorysize].get(token.clone()) {
                Some(_) => if memorysize == oringal_size {
                    return false
                }else{
                    return true
                },
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
        return self.return_switch
    }

    pub fn unset_return(&mut self) -> Object{
        self.return_switch = false;
        let value = self.return_value.clone();
        self.return_value = Object::NilObject;
        return value  
    }
}
