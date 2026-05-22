use crate::nodes::Node;
use crate::nodes::expression::{Expression, ExpressionNode};
use crate::nodes::lvalue::LValueNode;
use serde::Serialize;

#[derive(Serialize)]
pub enum Statement {
  Definition {
    is_constant: bool,
    is_export: bool,
    lhs: Vec<LValueNode>,
    rhs: Vec<ExpressionNode>,
  },
  Return(ExpressionNode),
  Break,
  Expression(Expression),
  SyntaxError(String),
}

pub type StatementNode = Node<Statement>;
