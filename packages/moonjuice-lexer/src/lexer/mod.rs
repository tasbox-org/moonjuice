use crate::TokenValue::UnexpectedCharacter;
use crate::{Lexer, Token, TokenValue};
use moonjuice_common::Position;
use moonjuice_common::peekable_stream::PeekableStream;
use std::ops::Range;

mod comments;
mod numerals;
mod operators;
mod symbols;

impl Lexer {
  pub(crate) fn new(source: Vec<char>) -> Self {
    Lexer {
      source: PeekableStream::new(source),
      position: Position { line: 1, column: 1 },
      token_start_index: 0,
      token_start_position: Position { line: 1, column: 1 },
    }
  }

  pub(crate) fn tokenise_next(&mut self) -> Option<Token> {
    while let Some(char) = self.source.peek_next()
      && char.is_whitespace()
    {
      self.advance();
    }

    if !self.source.has_next() {
      return None;
    }

    self.token_start_index = self.source.get_index();
    self.token_start_position = self.position.clone();

    let token = self
      .tokenise_comment()
      .or(self.tokenise_numeral())
      .or(self.tokenise_symbol());

    if token.is_none() {
      let character = self.source.peek_next().cloned().unwrap_or('\0');
      self.advance();

      Some(self.new_token(UnexpectedCharacter(character)))
    } else {
      token
    }
  }

  fn new_token(&self, value: TokenValue) -> Token {
    Token {
      value,
      lexeme: self.read_lexeme(),
      start: self.token_start_position.clone(),
      end: self.position.clone(),
    }
  }

  fn read_lexeme(&self) -> String {
    self.read_string(self.token_start_index..self.source.get_index())
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
    } else if char.is_some() {
      self.position.column += 1;
    }
  }
}
