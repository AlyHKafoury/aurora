use super::parser::expressions::{Expression, Object};

pub struct Interpreter {
    expressions: Vec<Expression>
}

impl Interpreter {
    pub fn new(expressions: Vec<Expression> ) -> Self {
        return Interpreter { expressions }
    }

    pub fn interpret(& self) -> Object {
        for expr in self.expressions.clone() {
            return expr.evaluate()
        }
        return Object::NilObject(None);
    }
}