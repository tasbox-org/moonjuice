use crate::nodes::Node;
use crate::nodes::lvalue::LValueNode;
use crate::nodes::statement::StatementNode;
use moonjuice_common::Operator;
use serde::Serialize;

#[derive(Serialize)]
pub enum TableDefinitionElement {
  Valid { key: ExpressionNode, value: ExpressionNode },
  SyntaxError(String),
}

pub type TableDefinitionElementNode = Node<TableDefinitionElement>;

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

pub type StringSegmentNode = Node<StringSegment>;

#[derive(Serialize)]
pub enum Expression {
  Nil,
  Bool(bool),
  Int(i64),
  Double(f64),
  String {
    segments: Vec<StringSegmentNode>,
    arguments: Vec<ExpressionNode>,
  },
  TableDefinition {
    elements: Vec<TableDefinitionElementNode>,
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

pub type ExpressionNode = Node<Expression>;
