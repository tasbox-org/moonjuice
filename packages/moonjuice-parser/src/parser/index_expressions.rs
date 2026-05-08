use crate::Parser;
use crate::nodes::expression::Expression::{BinaryOperator, SyntaxError};
use crate::nodes::expression::ExpressionNode;
use moonjuice_common::Operator::{Index, OptionalIndex};
use moonjuice_common::SpecialCharacter::{CloseSquareBracket, OpenSquareBracket};
use moonjuice_lexer::TokenValue::SpecialCharacter;

impl Parser {
  pub(super) fn parse_index_expression(&mut self, lhs: ExpressionNode, is_optional: bool) -> ExpressionNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != SpecialCharacter(OpenSquareBracket))
    {
      return ExpressionNode {
        value: SyntaxError("Expected '[' to open table index".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let rhs = self.parse_expression();

    if self
      .consume_if(|value| value == SpecialCharacter(CloseSquareBracket))
      .is_none()
    {
      return ExpressionNode {
        value: SyntaxError("Expected ']' to close table index".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let op = if is_optional { OptionalIndex } else { Index };

    ExpressionNode {
      value: BinaryOperator { op, lhs, rhs }.into(),
      start,
      end: self.get_end(),
    }
  }
}
