use moonjuice_common::peekable_stream::PeekableStream;
use moonjuice_common::{Keyword, Position, SpecialCharacter};

mod lexer;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(PartialEq, Debug, Clone)]
pub enum TokenValue {
  Nil,
  Bool(bool),
  Int(i64),
  Double(f64),
  String(String),
  FormatStringStart(String),
  FormatStringMiddle(String),
  FormatStringEnd(String),
  Symbol(String),
  Keyword(Keyword),
  Operator(Operator),
  SpecialCharacter(SpecialCharacter),
  Comment(String),
  UnexpectedCharacter(char),
  MalformedNumber(String),
  MalformedString(String),
}

#[derive(PartialEq, Debug)]
pub struct Token {
  pub value: TokenValue,
  pub lexeme: String,
  pub start: Position,
  pub end: Position,
}

pub struct Lexer {
  source: PeekableStream<char>,
  position: Position,

  token_start_index: usize,
  token_start_position: Position,
}

impl Lexer {
  pub fn tokenise(source: Vec<char>) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = vec![];

    while lexer.source.has_next() {
      if let Some(token) = lexer.tokenise_next() {
        tokens.push(token);
      }
    }

    tokens
  }
}
