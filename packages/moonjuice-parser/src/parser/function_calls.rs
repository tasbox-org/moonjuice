use crate::Parser;
use crate::nodes::expression::ExpressionNode;

impl Parser {
  pub(super) fn parse_call(&mut self, lhs: ExpressionNode, is_optional: bool) -> ExpressionNode {}
}
