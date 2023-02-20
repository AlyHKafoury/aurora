use super::parser::statements::Statement;

pub struct Interpreter {
    statments: Vec<Statement>
}

impl Interpreter {
    pub fn new(statments: Vec<Statement> ) -> Self {
        return Interpreter { statments }
    }

    pub fn interpret(&self) {
        for mut stmt in self.statments.clone() {
            stmt.evaluate()
        }
    }
}