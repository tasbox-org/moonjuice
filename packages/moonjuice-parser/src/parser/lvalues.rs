use crate::Parser;
use crate::nodes::lvalue::{LValue, LValueNode};
use moonjuice_common::Position;

impl Parser {
  pub(super) fn parse_lvalue(&mut self) -> LValueNode {
    LValueNode {
      value: LValue::SyntaxError("TODO".to_string()).into(),
      start: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 0 },
    }
  }
}
