use crate::Parser;
use crate::nodes::expression::{Expression, ExpressionNode};
use moonjuice_common::Position;

impl Parser {
  pub(super) fn parse_expression(&mut self) -> ExpressionNode {
    ExpressionNode {
      value: Expression::SyntaxError("TODO".to_string()).into(),
      start: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 0 },
    }
  }
}
