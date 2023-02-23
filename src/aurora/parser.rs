use crate::aurora::token;

use super::{expressions::Expression, expressions::Object, statements::Statement};

use super::token::TokenType;
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
        let expr = self.equality();

        if self.matches(Vec::from([TokenType::Equal])) {
            let value = self.assignment();

            match expr {
                Expression::Variable { name: n } => { 
                    return Expression::Assign { name: n, value: Box::new(value) }
                },
                _=> panic!("Invalid assignment {:?}", self.previous())
            }
        }
        return expr
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
        return self.primary();
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
            "Faild to Consume Correct token type {} {}",
            tokentype, message
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
        } else if self.matches(Vec::<TokenType>::from([TokenType::LeftBrace])) {
            return self.block();
        }

        return self.expr_statement();
    }

    fn block(&mut self) -> Statement {
        let mut stmnts = Vec::<Statement>::new();
        
        while !self.check(TokenType::RightBrace) && !self.at_end() {
            stmnts.push(self.declaration());
        }

        self.consume(TokenType::RightBrace, "expect '}' after block");

        return Statement::Block { statements: stmnts }
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
