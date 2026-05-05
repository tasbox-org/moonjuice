use crate::error::Error;
use crate::token::{Token, TokenValue};
use moonjuice_common::Position;
use moonjuice_common::peekable_stream::PeekableStream;
use std::ops::Range;

mod comments;

pub struct Lexer {
  source: PeekableStream<char>,
  position: Position,

  token_start_index: usize,
  token_start_position: Position,
}

impl Lexer {
  pub fn tokenise(source: Vec<char>) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(source);
    let mut tokens = vec![];

    while lexer.source.has_next() {
      if let Some(token) = lexer.tokenise_next()? {
        tokens.push(token);
      }
    }

    Ok(tokens)
  }

  fn new(source: Vec<char>) -> Self {
    Lexer {
      source: PeekableStream::new(source),
      position: Position { line: 1, column: 1 },
      token_start_index: 0,
      token_start_position: Position { line: 1, column: 1 },
    }
  }

  fn tokenise_next(&mut self) -> Result<Option<Token>, Error> {
    while let Some(char) = self.source.peek_next()
      && char.is_whitespace()
    {
      self.advance();
    }

    if !self.source.has_next() {
      return Ok(None);
    }

    self.token_start_index = self.source.get_index();
    self.token_start_position = self.position.clone();

    self.tokenise_comment()
  }

  fn new_token(&self, value: TokenValue) -> Token {
    Token {
      value,
      lexeme: self.read_string(self.token_start_index..self.source.get_index()),
      start: self.token_start_position.clone(),
      end: self.position.clone(),
    }
  }

  fn read_string(&self, range: Range<usize>) -> String {
    self.source.unwrap()[range].iter().collect()
  }

  fn advance_by(&mut self, n: usize) {
    for _ in 0..n {
      self.advance();
    }
  }

  fn advance(&mut self) {
    let char = self.source.consume().map(|char| *char);

    if char == Some('\n') {
      self.position.line += 1;
      self.position.column = 1;
    } else {
      self.position.column += 1;
    }
  }
}
