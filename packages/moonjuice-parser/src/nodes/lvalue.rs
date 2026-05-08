use crate::nodes::Node;
use crate::nodes::expression::ExpressionNode;
use moonjuice_common::Position;

pub enum TableUnpackElement {
  Valid { key: ExpressionNode, variable: LValueNode },
  SyntaxError(String),
}

pub enum LValue {
  Symbol(String),
  TableUnpack { elements: Vec<TableUnpackElement> },
  SyntaxError(String),
}

pub struct LValueNode {
  pub value: Box<LValue>,
  pub start: Position,
  pub end: Position,
}

impl Node for LValueNode {
  fn get_start(&self) -> Position {
    self.start.clone()
  }

  fn get_end(&self) -> Position {
    self.end.clone()
  }
}
