use super::{statements::Statement, environment::{Memory, Environment}};

pub struct Interpreter {
    statments: Vec<Statement>,
    env: Environment
}

impl Interpreter {
    pub fn new(statments: Vec<Statement> ) -> Self {
        return Interpreter { statments, env: Environment::new() }
    }

    pub fn interpret(&mut self) {
        for mut stmt in self.statments.clone() {
            stmt.evaluate(&mut self.env)
        }
    }
}