use crate::{Error, LuauTranspiler};
use moonjuice_common::{Operator, Position};
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
      Expression::String { segments, arguments } => {
        self.emit_string(segments, arguments, expression.start, expression.end)?
      }
      Expression::TableDefinition { elements } => {
        self.emit_table_definition(elements, expression.start, expression.end)?
      }
      Expression::Symbol(value) => self.emit_symbol(value.as_str()),
      Expression::Block(body) => self.emit_block(body)?,
      Expression::UnaryOperator { op, rhs } => self.emit_unary_operator(op, rhs, expression.start, expression.end)?,
      Expression::BinaryOperator { op, lhs, rhs } => {
        self.emit_binary_operator(op, lhs, rhs, expression.start, expression.end)?
      }
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

  fn emit_table_definition(
    &mut self,
    elements: Vec<TableDefinitionElement>,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    if !self.get_scope().is_in_expression {
      self.source.push_str("local _ = ");
    }

    self.push_expression_scope();
    self.source.push_str("{ ");

    self.emit_comma_separated(elements, |t, element| match element {
      TableDefinitionElement::Valid { key, value } => {
        t.source.push('[');
        t.emit_expression(key)?;
        t.source.push_str("] = ");
        t.emit_expression(value)
      }
      TableDefinitionElement::SyntaxError(message) => Err(Error {
        message,
        start: start.clone(),
        end: end.clone(),
      }),
    })?;

    self.source.push_str(" }");
    self.pop_scope();

    Ok(())
  }

  fn emit_unary_operator(
    &mut self,
    op: Operator,
    rhs: ExpressionNode,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    let (prefix, suffix) = match op {
      Operator::Subtract => ("(-", ")"),
      Operator::Not => ("(not ", ")"),
      Operator::BitwiseNot => ("bit32.bnot(", ")"),
      Operator::Length => ("(#", ")"),
      _ => {
        return Err(Error {
          message: "Invalid AST. UnaryOperator node contains a non-unary Operator (this should never happen!)"
            .to_string(),
          start,
          end,
        });
      }
    };

    if !self.get_scope().is_in_expression {
      self.source.push_str("local _ = ");
    }

    self.push_expression_scope();
    self.source.push_str(prefix);
    self.emit_expression(rhs)?;
    self.source.push_str(suffix);
    self.pop_scope();

    Ok(())
  }

  fn emit_binary_operator(
    &mut self,
    op: Operator,
    lhs: ExpressionNode,
    rhs: ExpressionNode,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    let (prefix, middle, suffix) = match op {
      Operator::Add => ("(", ") + (", ")"),
      Operator::Subtract => ("(", ") - (", ")"),
      Operator::Multiply => ("(", ") * (", ")"),
      Operator::Divide => ("(", ") / (", ")"),
      Operator::Modulo => ("(", ") % (", ")"),
      Operator::Concat => ("(", ") .. (", ")"),
      Operator::And => ("(", ") and (", ")"),
      Operator::Or => ("(", ") or (", ")"),
      Operator::OptionalCoalesce => (
        "(function() local lhs = (",
        "); return if lhs == nil then (",
        ") else lhs end)()",
      ),
      Operator::Equals => ("(", ") == (", ")"),
      Operator::NotEquals => ("(", ") ~= (", ")"),
      Operator::LessThan => ("(", ") < (", ")"),
      Operator::GreaterThan => ("(", ") > (", ")"),
      Operator::LessThanOrEqual => ("(", ") <= (", ")"),
      Operator::GreaterThanOrEqual => ("(", ") >= (", ")"),
      Operator::Pipe => {
        return self.emit_pipe_operator(lhs, rhs);
      }
      Operator::Index => ("(", ")[", "]"),
      Operator::OptionalIndex => (
        "(function() local lhs = ",
        "; return if lhs == nil then nil else lhs[",
        "] end)()",
      ),
      Operator::Assignment => {
        return self.emit_assignment_operator(lhs, rhs);
      }
      Operator::BitwiseAnd => ("bit32.band(", ", ", ")"),
      Operator::BitwiseOr => ("bit32.bor(", ", ", ")"),
      Operator::BitwiseXor => ("bit32.bxor(", ", ", ")"),
      Operator::LeftShift => ("bit32.lshift(", ", ", ")"),
      Operator::RightShift => ("bit32.rshift(", ", ", ")"),
      _ => {
        return Err(Error {
          message: "Invalid AST. BinaryOperator node contains a unary Operator (this should never happen!)".to_string(),
          start,
          end,
        });
      }
    };

    if !self.get_scope().is_in_expression {
      self.source.push_str("local _ = ");
    }

    self.push_expression_scope();
    self.source.push_str(prefix);
    self.emit_expression(lhs)?;
    self.source.push_str(middle);
    self.emit_expression(rhs)?;
    self.source.push_str(suffix);
    self.pop_scope();

    Ok(())
  }

  fn emit_assignment_operator(&mut self, lhs: ExpressionNode, rhs: ExpressionNode) -> Result<(), Error> {}

  fn emit_pipe_operator(&mut self, lhs: ExpressionNode, rhs: ExpressionNode) -> Result<(), Error> {}

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
