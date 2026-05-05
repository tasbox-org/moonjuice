use moonjuice_common::{Keyword, Operator, Position, SpecialCharacter};

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

pub struct Token {
  pub value: TokenValue,
  pub lexeme: String,
  pub start: Position,
  pub end: Position,
}
