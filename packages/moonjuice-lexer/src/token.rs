use moonjuice_common::Position;

pub enum TokenValue {
  Eof,
  Nil,
  Symbol(String),
  Comment(String),
  String(String),
}

pub struct Token {
  pub value: TokenValue,
  pub lexeme: String,
  pub start: Position,
  pub end: Position,
}
