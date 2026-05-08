use crate::nodes::Node;
use crate::nodes::lvalue::LValueNode;
use crate::nodes::statement::StatementNode;
use moonjuice_common::{Operator, Position};
use serde::Serialize;

#[derive(Serialize)]
pub enum TableDefinitionElement {
  Valid { key: ExpressionNode, value: ExpressionNode },
  SyntaxError(String),
}

#[derive(Serialize)]
pub struct IfBranch {
  pub condition: ExpressionNode,
  pub body: Vec<StatementNode>,
}

#[derive(Serialize)]
pub enum StringSegment {
  Valid(String),
  Malformed(String),
}

#[derive(Serialize)]
pub enum Expression {
  Nil,
  Bool(bool),
  Int(i64),
  Double(f64),
  String {
    segments: Vec<StringSegment>,
    arguments: Vec<ExpressionNode>,
  },
  TableDefinition {
    elements: Vec<TableDefinitionElement>,
  },
  Symbol(String),
  Block(Vec<StatementNode>),
  UnaryOperator {
    op: Operator,
    rhs: ExpressionNode,
  },
  BinaryOperator {
    op: Operator,
    lhs: ExpressionNode,
    rhs: ExpressionNode,
  },
  Function {
    parameters: Vec<LValueNode>,
    body: Vec<StatementNode>,
  },
  If {
    if_branches: Vec<IfBranch>,
    else_branch: Option<Vec<StatementNode>>,
  },
  For {
    lhs: Vec<LValueNode>,
    enumerable: ExpressionNode,
    body: Vec<StatementNode>,
  },
  Call {
    is_optional: bool,
    lhs: ExpressionNode,
    arguments: Vec<ExpressionNode>,
  },
  SyntaxError(String),
}

#[derive(Serialize)]
pub struct ExpressionNode {
  pub value: Box<Expression>,
  pub start: Position,
  pub end: Position,
}

impl Node for ExpressionNode {
  fn get_start(&self) -> Position {
    self.start.clone()
  }

  fn get_end(&self) -> Position {
    self.end.clone()
  }
}
