use std::collections::hash_map::HashMap;

use crate::aurora::token;

use super::token::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<token::Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(script: String) -> Scanner {
        let mut keywords = HashMap::<String, TokenType>::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("function"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);
        return Scanner {
            source: script,
            tokens: Vec::<token::Token>::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
            keywords: keywords.clone(),
        };
    }
    pub fn scan_tokens(&mut self) -> Vec<token::Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::Eof, String::new());
        return self.tokens.clone();
    }

    fn at_end(&self) -> bool {
        return self.current >= self.source.chars().count() ;
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self
            .source
            .chars()
            .nth(self.current - 1)
            .unwrap();
    }

    fn add_token(&mut self, tokentype: TokenType, literal: String) -> () {
        let text: String = self
            .source
            .chars()
            .skip(self.start )
            .take(self.current - self.start)
            .collect();
        self.tokens.push(token::Token {
            lexeme: text,
            tokentype: tokentype,
            literal: literal,
            line: self.line,
        })
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, String::new()),
            ')' => self.add_token(TokenType::RightParen, String::new()),
            '{' => self.add_token(TokenType::LeftBrace, String::new()),
            '}' => self.add_token(TokenType::RightBrace, String::new()),
            ',' => self.add_token(TokenType::Comma, String::new()),
            '.' => self.add_token(TokenType::Dot, String::new()),
            '-' => self.add_token(TokenType::Minus, String::new()),
            '+' => self.add_token(TokenType::Plus, String::new()),
            ';' => self.add_token(TokenType::SemiColon, String::new()),
            '*' => self.add_token(TokenType::Star, String::new()),
            '!' => match self.token_match('=') {
                true => self.add_token(TokenType::BangEqual, String::new()),
                false => self.add_token(TokenType::Bang, String::new()),
            },
            '=' => match self.token_match('=') {
                true => self.add_token(TokenType::EqualEqual, String::new()),
                false => self.add_token(TokenType::Equal, String::new()),
            },
            '<' => match self.token_match('=') {
                true => self.add_token(TokenType::LessEqual, String::new()),
                false => self.add_token(TokenType::Less, String::new()),
            },
            '>' => match self.token_match('=') {
                true => self.add_token(TokenType::GreaterEqual, String::new()),
                false => self.add_token(TokenType::Greater, String::new()),
            },
            '/' => match self.token_match('/') {
                true => {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                }
                false => {
                    self.add_token(TokenType::Slash, String::new());
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.read_string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                }else if self.is_alpha(c) {
                    self.identifier();
                }else {
                    println!("line : {} , unexpected character {}", self.line, c);
                    self.has_error = true;
                }
            }
        }
    }

    fn token_match(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current ).unwrap();
    }

    fn read_string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            print!("Unterminated string at {}", self.line);
            self.has_error = true;
            return;
        }

        self.advance();

        let text: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - (self.start + 2))
            .collect();
        self.add_token(TokenType::String, text);
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) -> () {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(
            TokenType::Number,
            self.source
                .chars()
                .skip(self.start )
                .take(self.current - self.start)
                .collect(),
        )
    }

    fn peek_next(&self) -> char {
        if (self.current + 1)  >= self.source.chars().count() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap();
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn identifier(&mut self) -> () {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text: String = self
            .source
            .chars()
            .skip(self.start )
            .take(self.current - self.start)
            .collect();
        let tokentype = self.keywords.get(&text).unwrap_or(&TokenType::Identifier);
        self.add_token(tokentype.to_owned(), String::new());
    }
}
