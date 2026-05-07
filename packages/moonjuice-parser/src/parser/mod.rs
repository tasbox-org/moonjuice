mod binary_operators;
mod breaks;
mod definitions;
mod expressions;
mod for_loops;
mod functions;
mod if_statements;
mod lvalues;
mod operands;
mod returns;
mod table_definitions;
mod unary_operators;

use crate::Parser;
use crate::nodes::expression::Expression::Block;
use crate::nodes::expression::ExpressionNode;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::{Break, Constant, Export, Mutable, Return};
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
    let start = self.get_start();

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

    ExpressionNode {
      value: Block(block).into(),
      start,
      end: self.get_end(),
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

  fn consume_if(&mut self, predicate: impl Fn(TokenValue) -> bool) -> Option<&Token> {
    self.tokens.consume_if(|token| predicate(token.value.clone()))
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
}
