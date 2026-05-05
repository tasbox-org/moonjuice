#[derive(Debug, Clone)]
pub struct Position {
  pub line: i32,
  pub column: i32,
}

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
  IndexExpression,
  OptionalIndexExpression,
  Call,
  OptionalCall,
  Assignment,
  BitwiseNot,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LeftShift,
  RightShift,
}

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
