use super::{statements::Statement, environment::Environment};

pub struct Interpreter {
    statments: Vec<Statement>,
    env: Environment
}

impl Interpreter {
    pub fn new(statments: Vec<Statement> ) -> Self {
        let mut env = Environment::new();
        env.global();
        return Interpreter { statments, env }
    }

    pub fn interpret(&mut self) {
        for stmt in self.statments.clone() {
            stmt.evaluate(&mut self.env)
        }
    }
}