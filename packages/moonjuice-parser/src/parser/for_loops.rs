use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::{Expression, ExpressionNode};
use moonjuice_common::Keyword::{Do, End, For, In};
use moonjuice_lexer::TokenValue::Keyword;

impl Parser {
  pub(super) fn parse_for(&mut self) -> ExpressionNode {
    let start = self.get_start();

    if self.tokens.consume().is_none_or(|token| token.value != Keyword(For)) {
      return ExpressionNode {
        value: SyntaxError("Expected 'for' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let lhs = self.consume_comma_separated(|p| p.parse_lvalue());

    if self.consume_if(Keyword(In)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected 'in' keyword to follow for loop variable declarations".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let enumerable = self.parse_expression();

    if self.consume_if(Keyword(Do)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected 'do' keyword to follow for loop enumerable".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let body = self.parse_block(|p| p.tokens.has_next() && !p.is_next(Keyword(End)));

    if self.consume_if(Keyword(End)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected 'end' keyword to close for loop body".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    ExpressionNode {
      value: Expression::For { lhs, enumerable, body }.into(),
      start,
      end: self.get_end(),
    }
  }
}
