use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType{
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }    
}

#[derive(Debug,Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub lexeme: String,
    pub tokentype: TokenType,
    pub literal: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token Type: {},  Lexeme: {}, String: {}, Line: {}", self.tokentype, self.lexeme, self.literal, self.line)
    }
}