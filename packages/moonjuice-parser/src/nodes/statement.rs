use crate::nodes::Node;
use crate::nodes::expression::{Expression, ExpressionNode};
use crate::nodes::lvalue::LValueNode;
use moonjuice_common::Position;
use serde::Serialize;

#[derive(Serialize)]
pub enum Statement {
  Definition {
    is_constant: bool,
    is_export: bool,
    definitions: Vec<(LValueNode, ExpressionNode)>,
  },
  Return(ExpressionNode),
  Break,
  Expression(Expression),
  SyntaxError(String),
}

#[derive(Serialize)]
pub struct StatementNode {
  pub value: Box<Statement>,
  pub start: Position,
  pub end: Position,
}

impl Node for StatementNode {
  fn get_start(&self) -> Position {
    self.start.clone()
  }

  fn get_end(&self) -> Position {
    self.end.clone()
  }
}
