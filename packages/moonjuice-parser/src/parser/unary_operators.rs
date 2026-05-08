use crate::Parser;
use crate::nodes::expression::Expression::{SyntaxError, UnaryOperator};
use crate::nodes::expression::ExpressionNode;
use crate::operators::{UNARY_PRECEDENCE, get_operator_metadata};
use moonjuice_common::Operator::Subtract;
use moonjuice_lexer::TokenValue::Operator;

impl Parser {
  pub(super) fn parse_unary_operator(&mut self) -> ExpressionNode {
    let start = self.get_start();
    let token = self.tokens.consume().cloned();
    let end = self.get_end();

    if let Some(token) = token
      && let Operator(op) = token.value.clone()
      && let Some(metadata) = get_operator_metadata(token)
      && (metadata.is_unary || op == Subtract)
    {
      let sub_expression_lhs = self.parse_operand();

      let rhs = self.parse_sub_expression(sub_expression_lhs, UNARY_PRECEDENCE);

      ExpressionNode {
        value: UnaryOperator { op, rhs }.into(),
        start,
        end,
      }
    } else {
      ExpressionNode {
        value: SyntaxError("Expected operand or unary operator".to_string()).into(),
        start,
        end,
      }
    }
  }
}
