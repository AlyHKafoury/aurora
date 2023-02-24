use crate::aurora::token;

use super::statements;
use super::{expressions::Expression, expressions::Object, statements::Statement};

use super::token::{Token, TokenType};
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

    fn expression(&mut self) -> Expression {
        return self.assignment();
    }

    fn assignment(&mut self) -> Expression {
        let expr = self.or();

        if self.matches(Vec::from([TokenType::Equal])) {
            let value = self.assignment();

            match expr {
                Expression::Variable { name: n } => {
                    return Expression::Assign {
                        name: n,
                        value: Box::new(value),
                    }
                }
                _ => panic!("Invalid assignment {:?}", self.previous()),
            }
        }
        return expr;
    }

    fn or(&mut self) -> Expression {
        let mut expr = self.and();

        while self.matches(Vec::from([TokenType::Or])) {
            let operator = self.previous();
            let right = self.and();
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return expr;
    }

    fn and(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.matches(Vec::from([TokenType::And])) {
            let operator = self.previous();
            let right = self.equality();
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        return expr;
    }

    fn equality(&mut self) -> Expression {
        println!("in equality");
        let mut expr = self.comparison();

        while self.matches(Vec::from([TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn comparison(&mut self) -> Expression {
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
            expr = Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn term(&mut self) -> Expression {
        println!("in term");
        let mut expr = self.factor();

        while self.matches(Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn factor(&mut self) -> Expression {
        println!("in factor");
        let mut expr = self.unary();

        while self.matches(Vec::from([TokenType::Slash, TokenType::Star])) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary {
                left: Box::new(expr.clone()),
                operator: operator,
                right: Box::new(right),
            };
        }
        return expr;
    }

    fn unary(&mut self) -> Expression {
        println!("in unary");
        if self.matches(Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = self.previous();
            let right = self.unary();
            return Expression::Unary {
                operator: operator,
                right: Box::new(right),
            };
        }
        return self.call();
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary();

        loop {
            if self.matches(vec![TokenType::LeftParen]) {
                expr = self.do_call(&mut expr);
            } else {
                break;
            }
        }

        return expr;
    }

    fn do_call(&mut self, callee: &mut Expression) -> Expression {
        let mut arguments = Vec::<Expression>::new();

        if !self.check(TokenType::RightParen) {
            if arguments.len() >= 255 {
                panic!(
                    "cannot have more than 255 arguements to a function {}",
                    self.peek()
                );
            }
            arguments.push(self.expression());
            while self.matches(vec![TokenType::Comma]) {
                arguments.push(self.expression());
            }
        }

        let paren = self.consume(TokenType::RightParen, "expect ')' after arguments");
        return Expression::Call {
            callee: Box::new(callee.clone()),
            paren,
            arguments,
        };
    }

    fn primary(&mut self) -> Expression {
        println!("in primary");
        if self.matches(Vec::from([TokenType::False])) {
            return Expression::Literal {
                value: Object::BoolObject(false),
            };
        }
        if self.matches(Vec::from([TokenType::True])) {
            return Expression::Literal {
                value: Object::BoolObject(true),
            };
        }
        if self.matches(Vec::from([TokenType::Nil])) {
            return Expression::Literal {
                value: Object::NilObject,
            };
        }
        if self.matches(Vec::from([TokenType::Number, TokenType::String])) {
            return Expression::Literal {
                value: match self.previous().tokentype {
                    TokenType::String => Object::StringObject(self.previous().literal),
                    TokenType::Number => {
                        Object::NumberObject(self.previous().literal.parse::<f64>().unwrap())
                    }
                    _ => {
                        panic!("Token Not Number or String! {}", self.previous())
                    }
                },
            };
        }
        if self.matches(Vec::from([TokenType::Identifier])) {
            return Expression::Variable {
                name: self.previous(),
            };
        }
        if self.matches(Vec::from([TokenType::LeftParen])) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ) after expression");
            return Expression::Grouping {
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
        panic!(
            "Faild to Consume Correct token type {}, {}. {}",
            tokentype, message, self.current
        );
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
        } else if self.check(TokenType::LeftBrace) {
            return self.block();
        } else if self.matches(Vec::<TokenType>::from([TokenType::If])) {
            return self.if_statement();
        } else if self.matches(Vec::<TokenType>::from([TokenType::While])) {
            return self.while_statment();
        } else if self.matches(Vec::<TokenType>::from([TokenType::For])) {
            return self.for_statement();
        }   else if self.matches(Vec::<TokenType>::from([TokenType::Fun])) {
            return self.function("function".to_string());
        }   else if self.matches(Vec::<TokenType>::from([TokenType::Return])) {
            return self.return_statement();
        }

        return self.expr_statement();
    }

    fn return_statement(&mut self) -> Statement {
        let keyword = self.previous();

        let mut value = None;
        if !self.check(TokenType::SemiColon) {
            value = Some(self.expression());
        }

        self.consume(TokenType::SemiColon, "expected semicolon after return");
        return Statement::Return { keyword, value: value }
    }

    fn function(&mut self, functype: String) -> Statement {
        let name = self.consume(
            TokenType::Identifier,
            format!("expect {} name", &functype).as_str(),
        );

        self.consume(
            TokenType::LeftParen,
            format!("expect ( after {} name", &functype).as_str(),
        );

        let mut params = Vec::<Token>::new();
        if !self.check(TokenType::RightParen) {
            if params.len() >= 255 {
                panic!(
                    "cannot have more than 255 arguements to a function {}",
                    self.peek()
                );
            }
            params.push(self.consume(
                TokenType::Identifier,
                format!("Expected Identifier in Params {}", self.previous()).as_str(),
            ));
            while self.matches(vec![TokenType::Comma]) {
                params.push(self.consume(
                    TokenType::Identifier,
                    format!("Expected Identifier in Params {}", self.previous()).as_str(),
                ));
            }
        }
        self.consume(
            TokenType::RightParen,
            format!("expect ) after {} params", &functype).as_str(),
        );

        let body = self.block();
        return Statement::Function {
            name,
            params,
            body: Box::new(body),
        };
    }

    fn for_statement(&mut self) -> Statement {
        self.consume(TokenType::LeftParen, "expect '(' after 'if'");
        let init;
        if self.matches(vec![TokenType::SemiColon]) {
            init = None;
        } else if self.matches(vec![TokenType::Var]) {
            init = Some(self.var_declaration());
        } else {
            init = Some(self.expr_statement());
        }

        let mut condition = None;
        if !self.check(TokenType::SemiColon) {
            condition = Some(self.expression());
        }

        self.consume(TokenType::SemiColon, "expect ';' after loop condition");

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression());
        }

        self.consume(TokenType::RightParen, "expect ')' after 'loop' clauses");

        let body = self.block();

        return Statement::For {
            init: Box::new(init),
            condition,
            increment,
            body: Box::new(body),
        };
    }

    fn while_statment(&mut self) -> Statement {
        self.consume(TokenType::LeftParen, "expect '(' after 'if'");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "expect ')' after 'if'");
        let body = self.block();

        return Statement::While {
            condition,
            body: Box::new(body),
        };
    }

    fn if_statement(&mut self) -> Statement {
        self.consume(TokenType::LeftParen, "expect '(' after 'if'");
        let condition = self.expression();
        self.consume(TokenType::RightParen, "expect ')' after 'if'");

        let then_branch = Box::new(self.block());
        let else_branch = match self.matches(Vec::<TokenType>::from([TokenType::Else])) {
            true => Some(Box::new(self.block())),
            false => None,
        };

        return Statement::If {
            condition,
            then_branch,
            else_branch,
        };
    }

    fn block(&mut self) -> Statement {
        let mut stmnts = Vec::<Statement>::new();
        self.consume(TokenType::LeftBrace, "expect '{' before block");

        while !self.check(TokenType::RightBrace) && !self.at_end() {
            stmnts.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "expect '}' after block");

        return Statement::Block { statements: stmnts };
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
        return Statement::Variable { name: name, init };
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::<Statement>::new();
        while !self.at_end() {
            statements.push(self.declaration());
        }

        return statements;
    }
}
