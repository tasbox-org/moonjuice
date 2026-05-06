pub mod peekable_stream;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
