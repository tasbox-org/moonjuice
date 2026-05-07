use crate::nodes::Node;
use crate::nodes::expression::ExpressionNode;
use moonjuice_common::Position;

pub struct TableUnpackElement {
  pub key: ExpressionNode,
  pub variable: LValueNode,
}

pub enum LValue {
  Symbol(String),
  TableUnpack { elements: Vec<TableUnpackElement> },
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
