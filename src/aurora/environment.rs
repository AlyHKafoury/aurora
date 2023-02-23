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

    pub fn get(& self, token: Token) -> Option<Object> {
       match self.0.get(&token.lexeme) {
        Some(x) => return Some(x.clone()),
        None => None
       }
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), ()> {
        match self.0.get(&token.lexeme) {
         Some(_) => {self.0.insert(token.lexeme.clone(), value); return Ok(())},
         None => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment(Vec<Memory>);

impl Environment {
    pub fn new() -> Self {
        let mut memorystack = Vec::<Memory>::new();
        memorystack.push(Memory::new());
        return Environment(memorystack);
    }

    pub fn stackpush(&mut self) {
        self.0.push(Memory::new())
    }

    pub fn stackpop(&mut self) {
        self.0.pop();
    }

    pub fn define(&mut self, k: Token, v: Object) {
        let memorysize = self.0.len()-1;
        self.0[memorysize].define(k, v);
    }

    pub fn get(& self, token: Token) -> Object {
        let mut memorysize = self.0.len()-1;
        
        while memorysize >= 0 {
            match self.0[memorysize].get(token.clone()) {
                Some(x) => return x,
                _ => memorysize -= 1
            }
        }

        panic!("Undefined variable {}", token.lexeme);
     }

     pub fn assign(&mut self, token: Token, value: Object) {
        let mut memorysize = self.0.len()-1;

        while memorysize >= 0 { 
            match self.0[memorysize].assign(token.clone(), value.clone()) {
                Ok(_) => return,
                _ => memorysize -= 1
            }
        }

        panic!("Undefined variable {}", token.lexeme);
     }

}

