use crate::aurora::token;

use self::{statements::Statement, expressions::Expression};

use super::token::TokenType;
pub mod expressions;
pub mod statements;
pub struct Parser {
    pub tokens: Vec<token::Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Self {
        return Parser {
            tokens: tokens,
            current: 0,
        };
    }

    fn advance(&mut self) -> token::Token {
        if !self.at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn peek(&self) -> token::Token {
        // if self.current >= self.tokens.iter().count() {
        //     return token::Token{ lexeme: "EOF".to_owned(), tokentype: TokenType::Eof, literal: "".to_owned(), line: 0 };
        // }
        let tokens = self.tokens.clone();
        return tokens.into_iter().nth(self.current).unwrap();
    }
    fn previous(&self) -> token::Token {
        let tokens = self.tokens.clone();
        return tokens.into_iter().nth(self.current - 1).unwrap();
    }
    fn at_end(&self) -> bool {
        return self.peek().tokentype.clone() == token::TokenType::Eof;
    }
    fn check(&self, tokentype: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        return self.peek().tokentype == tokentype;
    }
    fn matches(&mut self, tokentypes: Vec<token::TokenType>) -> bool {
        for tokentype in tokentypes {
            if self.check(tokentype) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn expression(&mut self) -> expressions::Expression {
        return self.equality();
    }

    fn equality(&mut self) -> expressions::Expression {
        println!("in equality");
        let mut expr = self.comparison();

        while self.matches(Vec::from([TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = self.previous();
            let right = self.comparison();
            expr = expressions::Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn comparison(&mut self) -> expressions::Expression {
        println!("in comp");
        let mut expr = self.term();

        while self.matches(Vec::from([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])) {
            let operator = self.previous();
            let right = self.term();
            expr = expressions::Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn term(&mut self) -> expressions::Expression {
        println!("in term");
        let mut expr = self.factor();

        while self.matches(Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator = self.previous();
            let right = self.factor();
            expr = expressions::Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn factor(&mut self) -> expressions::Expression {
        println!("in factor");
        let mut expr = self.unary();

        while self.matches(Vec::from([TokenType::Slash, TokenType::Star])) {
            let operator = self.previous();
            let right = self.unary();
            expr = expressions::Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn unary(&mut self) -> expressions::Expression {
        println!("in unary");
        if self.matches(Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = self.previous();
            let right = self.unary();
            return expressions::Expression::Unary {
                operator: operator,
                right: Box::new(right),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> expressions::Expression {
        println!("in primary");
        if self.matches(Vec::from([TokenType::False])) {
            return expressions::Expression::Literal {
                value: expressions::Object::BoolObject(false),
            };
        }
        if self.matches(Vec::from([TokenType::True])) {
            return expressions::Expression::Literal {
                value: expressions::Object::BoolObject(true),
            };
        }
        if self.matches(Vec::from([TokenType::Nil])) {
            return expressions::Expression::Literal {
                value: expressions::Object::NilObject(None),
            };
        }
        if self.matches(Vec::from([TokenType::Number, TokenType::String])) {
            return expressions::Expression::Literal {
                value: match self.previous().tokentype {
                    TokenType::String => expressions::Object::StringObject(self.previous().literal),
                    TokenType::Number => expressions::Object::NumberObject(
                        self.previous().literal.parse::<f64>().unwrap(),
                    ),
                    _ => {panic!("Token Not Number or String! {}", self.previous())}
                },
            };
        }
        if self.matches(Vec::from([TokenType::Var])){
            return Expression::Variable { name: self.previous() }
        }
        if self.matches(Vec::from([TokenType::LeftParen])) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ) after expression");
            return expressions::Expression::Grouping {
                expression: Box::new(expr),
            };
        }
        panic!(
            "Faild to Parse Primary Expression {}",
            self.tokens.iter().nth(self.current).unwrap()
        )
    }

    fn consume(&mut self, tokentype: TokenType, message: &str) -> token::Token {
        if self.check(tokentype.clone()) {
            return self.advance();
        }
        panic!("Faild to Consume Correct token type {} {}", tokentype, message);
    }

    fn synchronize(&mut self) -> () {
        self.advance();

        while !self.at_end() {
            if self.previous().tokentype == TokenType::SemiColon {
                return;
            }

            match self.peek().tokentype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn print_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume(TokenType::SemiColon, "expected ; after print value");

        return Statement::Print { expression: expr };
    }

    fn expr_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume(TokenType::SemiColon, "expected ; after expression");

        return Statement::Expression { expression: expr };
    }

    fn statement(&mut self) -> Statement {
        if self.matches(Vec::<TokenType>::from([TokenType::Print])) {
            return self.print_statement();
        }

        return self.expr_statement();
    }

    fn declaration(&mut self) -> Statement {
        if self.matches(Vec::<TokenType>::from([TokenType::Var])) {
            return self.var_declaration();
        }
        return self.statement();
    }

    fn var_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "expected variable name");

        let mut init = None;
        if self.matches(Vec::<TokenType>::from([TokenType::Equal])) {
            init = Some(self.expression());
        }

        self.consume(TokenType::SemiColon, "expected semicolon after initlizer");
        return Statement::Variable { name: name, init: init };
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::<Statement>::new();
        while !self.at_end() {
            statements.push(self.declaration());
        }

        return statements;
    }
}
