use crate::nodes::Node;
use crate::nodes::expression::ExpressionNode;
use crate::nodes::lvalue::LValueNode;
use moonjuice_common::Position;

pub enum Statement {
  Definition {
    is_constant: bool,
    is_export: bool,
    lhs: LValueNode,
    rhs: ExpressionNode,
  },
  Return(ExpressionNode),
  Break,
  Expression(ExpressionNode),
}

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
