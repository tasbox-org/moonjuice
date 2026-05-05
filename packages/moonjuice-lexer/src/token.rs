use moonjuice_common::{Keyword, Operator, Position, SpecialCharacter};

#[derive(PartialEq, Debug)]
pub enum TokenValue {
  Eof,
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
}

#[derive(PartialEq, Debug)]
pub struct Token {
  pub value: TokenValue,
  pub lexeme: String,
  pub start: Position,
  pub end: Position,
}
