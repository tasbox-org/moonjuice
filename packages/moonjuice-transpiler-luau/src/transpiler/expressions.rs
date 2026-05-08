use crate::{Error, LuauTranspiler};
use moonjuice_common::Operator;
use moonjuice_parser::nodes::expression::{Expression, ExpressionNode, IfBranch, TableDefinitionElement};
use moonjuice_parser::nodes::lvalue::LValueNode;
use moonjuice_parser::nodes::statement::StatementNode;

impl LuauTranspiler {
  pub(super) fn emit_expression(&mut self, expression: ExpressionNode) -> Result<(), Error> {
    match *expression.value {
      Expression::Nil => self.emit_nil(),
      Expression::Bool(value) => self.emit_bool(value),
      Expression::Int(value) => self.emit_int(value),
      Expression::Double(value) => self.emit_double(value),
      Expression::String { segments, arguments } => self.emit_string(segments, arguments)?,
      Expression::TableDefinition { elements } => self.emit_table_definition(elements)?,
      Expression::Symbol(value) => self.emit_symbol(value.as_str()),
      Expression::Block(body) => self.emit_block(body)?,
      Expression::UnaryOperator { op, rhs } => self.emit_unary_operator(op, rhs)?,
      Expression::BinaryOperator { op, lhs, rhs } => self.emit_binary_operator(op, lhs, rhs)?,
      Expression::Function { parameters, body } => self.emit_function(parameters, body)?,
      Expression::If {
        if_branches,
        else_branch,
      } => self.emit_if(if_branches, else_branch)?,
      Expression::For { lhs, enumerable, body } => self.emit_for(lhs, enumerable, body)?,
      Expression::Call {
        is_optional,
        lhs,
        arguments,
      } => self.emit_call(is_optional, lhs, arguments)?,
      Expression::SyntaxError(message) => {
        return Err(Error {
          message,
          start: expression.start,
          end: expression.end,
        });
      }
    }

    Ok(())
  }

  fn emit_table_definition(&mut self, elements: Vec<TableDefinitionElement>) -> Result<(), Error> {}

  fn emit_unary_operator(&mut self, op: Operator, rhs: ExpressionNode) -> Result<(), Error> {}

  fn emit_binary_operator(&mut self, op: Operator, lhs: ExpressionNode, rhs: ExpressionNode) -> Result<(), Error> {}

  fn emit_function(&mut self, parameters: Vec<LValueNode>, body: Vec<StatementNode>) -> Result<(), Error> {}

  fn emit_if(&mut self, if_branches: Vec<IfBranch>, else_branch: Option<Vec<StatementNode>>) -> Result<(), Error> {}

  fn emit_for(
    &mut self,
    lhs: Vec<LValueNode>,
    enumerable: ExpressionNode,
    body: Vec<StatementNode>,
  ) -> Result<(), Error> {
  }

  fn emit_call(&mut self, is_optional: bool, lhs: ExpressionNode, arguments: Vec<ExpressionNode>) -> Result<(), Error> {
  }
}
