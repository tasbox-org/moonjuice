mod binary_operators;
mod breaks;
mod definitions;
mod expressions;
mod for_loops;
mod function_calls;
mod function_definitions;
mod if_expressions;
mod index_expressions;
mod lvalues;
mod operands;
mod returns;
mod strings;
mod table_definitions;
mod table_unpacks;
mod unary_operators;

use crate::Parser;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::{Break, Constant, Export, Mutable, Return};
use moonjuice_common::Position;
use moonjuice_common::SpecialCharacter::Comma;
use moonjuice_common::peekable_stream::PeekableStream;
use moonjuice_lexer::TokenValue::{Keyword, SpecialCharacter};
use moonjuice_lexer::{Token, TokenValue};

impl Parser {
  pub(crate) fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens: PeekableStream::new(
        tokens
          .into_iter()
          .filter(|token| !matches!(token.value, TokenValue::Comment(_)))
          .collect(),
      ),
    }
  }

  pub(crate) fn parse_block(&mut self, has_remaining: impl Fn(&Self) -> bool) -> Vec<StatementNode> {
    let mut block: Vec<StatementNode> = vec![];

    while has_remaining(self) {
      let node = if self.is_next(Keyword(Constant)) || self.is_next(Keyword(Mutable)) || self.is_next(Keyword(Export)) {
        self.parse_definition()
      } else if self.is_next(Keyword(Return)) {
        self.parse_return()
      } else if self.is_next(Keyword(Break)) {
        self.parse_break()
      } else {
        // TODO #156: Handle multiple return
        let expr = self.parse_expression();

        StatementNode {
          value: Statement::Expression(*expr.value).into(),
          start: expr.start,
          end: expr.end,
        }
      };

      block.push(node);
    }

    block
  }

  fn is_next(&self, value: TokenValue) -> bool {
    match self.tokens.peek_next() {
      Some(Token {
        value: actual_value, ..
      }) => *actual_value == value,
      _ => false,
    }
  }

  fn consume_if(&mut self, value: TokenValue) -> Option<&Token> {
    self.tokens.consume_if(|token| token.value == value)
  }

  fn get_start(&self) -> Position {
    self
      .tokens
      .peek_next()
      .map(|token| token.start.clone())
      .or_else(|| self.tokens.peek_back(1).map(|token| token.end.clone()))
      .unwrap_or(Position { line: 1, column: 1 })
  }

  fn get_end(&self) -> Position {
    self
      .tokens
      .peek_back(1)
      .map(|token| token.end.clone())
      .unwrap_or(Position { line: 1, column: 1 })
  }

  fn consume_comma_separated<T>(&mut self, for_each: impl Fn(&mut Self) -> T) -> Vec<T> {
    let mut elements: Vec<T> = vec![];

    loop {
      elements.push(for_each(self));

      if self.consume_if(SpecialCharacter(Comma)).is_none() {
        break elements;
      }
    }
  }

  fn consume_trailing_comma_separated<T>(&mut self, stop_on: TokenValue, for_each: impl Fn(&mut Self) -> T) -> Vec<T> {
    let mut elements: Vec<T> = vec![];

    loop {
      if self.is_next(stop_on.clone()) {
        break elements;
      }

      elements.push(for_each(self));

      if self.consume_if(SpecialCharacter(Comma)).is_none() {
        break elements;
      }
    }
  }
}
