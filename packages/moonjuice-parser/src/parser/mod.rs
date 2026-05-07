mod definitions;
mod expressions;
mod returns;

use crate::Parser;
use crate::nodes::expression::Expression::Block;
use crate::nodes::expression::ExpressionNode;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::{Constant, Export, Mutable, Return};
use moonjuice_common::Position;
use moonjuice_common::peekable_stream::PeekableStream;
use moonjuice_lexer::TokenValue::Keyword;
use moonjuice_lexer::{Token, TokenValue};

impl Parser {
  pub(crate) fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens: PeekableStream::new(tokens),
    }
  }

  pub(crate) fn parse_block(&mut self, has_remaining: impl Fn(&Self) -> bool) -> ExpressionNode {
    let mut block: Vec<StatementNode> = vec![];
    let start = self
      .tokens
      .peek_next()
      .map(|token| token.start.clone())
      .or_else(|| self.tokens.peek_back(1).map(|token| token.end.clone()))
      .unwrap_or(Position { line: 1, column: 1 });

    while has_remaining(self) {
      let node = if self.is_next(Keyword(Constant)) || self.is_next(Keyword(Mutable)) || self.is_next(Keyword(Export)) {
        self.parse_definition()
      } else if self.is_next(Keyword(Return)) {
        self.parse_return()
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

    let end = self
      .tokens
      .peek_back(1)
      .map(|token| token.end.clone())
      .unwrap_or(Position { line: 1, column: 1 });

    ExpressionNode {
      value: Block(block).into(),
      start,
      end,
    }
  }

  fn is_next(&self, value: TokenValue) -> bool {
    match self.tokens.peek_next() {
      Some(Token {
        value: actual_value, ..
      }) => *actual_value == value,
      _ => false,
    }
  }
}
