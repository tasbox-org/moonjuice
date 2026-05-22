use crate::nodes::Node;
use crate::nodes::expression::ExpressionNode;
use moonjuice_common::Position;
use serde::Serialize;

#[derive(Serialize)]
pub enum TableUnpackElement {
  Valid { key: ExpressionNode, variable: LValueNode },
  SyntaxError(String),
}

pub type TableUnpackElementNode = Node<TableUnpackElement>;

#[derive(Serialize)]
pub enum LValue {
  Symbol(String),
  TableUnpack { elements: Vec<TableUnpackElementNode> },
  SyntaxError(String),
}

pub type LValueNode = Node<LValue>;
