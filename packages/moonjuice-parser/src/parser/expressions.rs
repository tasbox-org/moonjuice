use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::ExpressionNode;
use crate::operators::get_operator_metadata;
use moonjuice_common::Operator::OptionalIndex;
use moonjuice_common::SpecialCharacter::{OpenBracket, OpenSquareBracket};
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::{Operator, SpecialCharacter};

fn is_binary_operator_with_greater_or_equal_precedence(token: Token, min_precedence: u16) -> bool {
  if let Some(metadata) = get_operator_metadata(token)
    && !metadata.is_unary
    && metadata.precedence >= min_precedence
  {
    true
  } else {
    false
  }
}

impl Parser {
  pub(super) fn parse_expression(&mut self) -> ExpressionNode {
    let lhs = self.parse_operand();

    match lhs.value.as_ref() {
      SyntaxError(_) => lhs,
      _ => self.parse_sub_expression(lhs, 0),
    }
  }

  pub(super) fn parse_sub_expression(&mut self, mut lhs: ExpressionNode, min_precedence: u16) -> ExpressionNode {
    while let Some(lookahead) = self.tokens.peek_next()
      && is_binary_operator_with_greater_or_equal_precedence(lookahead.clone(), min_precedence)
    {
      lhs = match lookahead.value {
        SpecialCharacter(OpenBracket) => self.parse_call(lhs, false),
        SpecialCharacter(OpenSquareBracket) => self.parse_index_expression(lhs, false),
        Operator(OptionalIndex)
          if self
            .tokens
            .peek(1)
            .is_some_and(|token| token.value == SpecialCharacter(OpenBracket)) =>
        {
          self.tokens.consume();
          self.parse_call(lhs, true)
        }
        Operator(OptionalIndex)
          if self
            .tokens
            .peek(1)
            .is_some_and(|token| token.value == SpecialCharacter(OpenSquareBracket)) =>
        {
          self.tokens.consume();
          self.parse_index_expression(lhs, true)
        }
        Operator(_) => self.parse_binary_operator(lhs),
        _ => panic!(
          "is_binary_operator_with_greater_or_equal_precedence returned true for token other than binary operator"
        ),
      }
    }

    lhs
  }
}
