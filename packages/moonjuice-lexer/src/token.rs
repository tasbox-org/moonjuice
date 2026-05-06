use moonjuice_common::{Keyword, Operator, Position, SpecialCharacter};

#[derive(PartialEq, Debug, Clone)]
pub enum TokenValue {
  Nil,
  Bool(bool),
  Int(i64),
  Double(f64),
  String(String),
  Symbol(String),
  Keyword(Keyword),
  Operator(Operator),
  SpecialCharacter(SpecialCharacter),
  Comment(String),
  UnexpectedCharacter(char),
  MalformedNumber(String),
}

#[derive(PartialEq, Debug)]
pub struct Token {
  pub value: TokenValue,
  pub lexeme: String,
  pub start: Position,
  pub end: Position,
}
