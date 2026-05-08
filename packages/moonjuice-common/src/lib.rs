use serde::Serialize;

pub mod peekable_stream;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Keyword {
  Break,
  Continue,
  Return,

  Do,
  End,

  Function,

  If,
  Then,
  Else,
  ElseIf,

  For,
  In,

  Constant,
  Mutable,
  Export,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SpecialCharacter {
  OpenBracket,
  CloseBracket,

  OpenSquareBracket,
  CloseSquareBracket,

  OpenCurlyBracket,
  CloseCurlyBracket,

  Comma,
  Colon,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Operator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Concat,
  Length,
  Not,
  And,
  Or,
  OptionalCoalesce,
  Equals,
  NotEquals,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
  Pipe,
  Index,
  OptionalIndex,
  Assignment,
  BitwiseNot,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LeftShift,
  RightShift,
}
