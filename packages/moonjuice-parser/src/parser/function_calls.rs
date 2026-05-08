use crate::Parser;
use crate::nodes::expression::Expression::{Call, SyntaxError};
use crate::nodes::expression::ExpressionNode;
use moonjuice_common::SpecialCharacter::{CloseBracket, OpenBracket};
use moonjuice_lexer::TokenValue::SpecialCharacter;

impl Parser {
  pub(super) fn parse_call(&mut self, lhs: ExpressionNode, is_optional: bool) -> ExpressionNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != SpecialCharacter(OpenBracket))
    {
      return ExpressionNode {
        value: SyntaxError("Expected '(' to open function argument list".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let arguments = if self.is_next(SpecialCharacter(CloseBracket)) {
      vec![]
    } else {
      self.consume_comma_separated(|p| p.parse_expression())
    };

    if self.consume_if(SpecialCharacter(CloseBracket)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected ')' to close function argument list".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    ExpressionNode {
      value: Call {
        lhs,
        is_optional,
        arguments,
      }
      .into(),
      start,
      end: self.get_end(),
    }
  }
}
