use crate::{Error, LuauTranspiler};
use moonjuice_common::{Operator, Position};
use moonjuice_parser::nodes::expression::Expression::Call;
use moonjuice_parser::nodes::expression::{
  Expression, ExpressionNode, IfBranch, StringSegment, TableDefinitionElement,
};
use moonjuice_parser::nodes::lvalue::LValueNode;
use moonjuice_parser::nodes::statement::StatementNode;
use std::fmt::Write;

fn is_valid_lua_symbol(string: &String) -> bool {
  if string.chars().next().is_none_or(|char| char.is_ascii_digit()) {
    return false;
  }

  for char in string.chars() {
    if !char.is_ascii_alphanumeric() && char != '_' {
      return false;
    }
  }

  true
}

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
      Call {
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
      TableDefinitionElement::SyntaxError(message) => Err(Error { message, start, end }),
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
        return self.emit_pipe_operator(lhs, rhs, start, end);
      }
      Operator::Index => {
        return self.emit_index_operator(lhs, rhs);
      }
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

  fn emit_index_operator(&mut self, lhs: ExpressionNode, rhs: ExpressionNode) -> Result<(), Error> {
    self.push_expression_scope();

    match *lhs.value {
      Expression::Symbol(symbol) => {
        self.source.push_str(symbol.as_str());
      }
      Expression::BinaryOperator {
        op: Operator::Index,
        lhs,
        rhs,
        ..
      } => {
        self.emit_index_operator(lhs, rhs)?;
      }
      _ => {
        self.source.push_str("(");
        self.emit_expression(lhs)?;
        self.source.push_str(")");
      }
    }

    match &*rhs.value {
      Expression::String { segments, .. } if segments.len() == 1 => match segments.first() {
        Some(StringSegment::Valid(string)) if is_valid_lua_symbol(string) => {
          self.source.push('.');
          self.source.push_str(string);
        }
        _ => {
          self.source.push_str("[");
          self.emit_expression(rhs)?;
          self.source.push_str("]");
        }
      },
      _ => {
        self.source.push_str("[");
        self.emit_expression(rhs)?;
        self.source.push_str("]");
      }
    }

    self.pop_scope();

    Ok(())
  }

  fn emit_assignment_operator(&mut self, lhs: ExpressionNode, rhs: ExpressionNode) -> Result<(), Error> {
    self.push_expression_scope();

    if self.get_scope().is_in_expression {
      let ret_symbol = format!("ret_{}", self.get_unique_id());

      write!(self.source, "(function()\nlocal {} = ", ret_symbol).ok();
      self.emit_expression(rhs)?;
      self.source.push_str(";\n");

      self.emit_expression(lhs)?;
      write!(self.source, " = {};\nreturn {} end)()", ret_symbol, ret_symbol).ok();
    } else {
      self.emit_expression(lhs)?;
      self.source.push_str(" = ");
      self.emit_expression(rhs)?;
    }

    self.pop_scope();

    Ok(())
  }

  fn emit_pipe_operator(
    &mut self,
    lhs: ExpressionNode,
    rhs: ExpressionNode,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    let piped_call = if let Call {
      is_optional,
      lhs: original_lhs,
      mut arguments,
    } = *rhs.value
    {
      arguments.splice(0..0, vec![lhs]);

      Call {
        is_optional,
        lhs: original_lhs,
        arguments,
      }
    } else {
      Call {
        is_optional: false,
        lhs: rhs,
        arguments: vec![lhs],
      }
    };

    self.emit_expression(ExpressionNode {
      value: piped_call.into(),
      start,
      end,
    })
  }

  fn emit_function(&mut self, parameters: Vec<LValueNode>, body: Vec<StatementNode>) -> Result<(), Error> {
    if !self.get_scope().is_in_expression {
      return Ok(());
    }

    self.source.push_str("function(");
    self.push_lvalue_scope();

    self.emit_comma_separated(parameters, Self::emit_lvalue)?;
    let table_unpacks = std::mem::take(self.get_scope_mut().table_unpacks.as_mut());

    self.pop_scope();
    self.source.push_str(")\n");

    self.emit_table_unpacks(table_unpacks)?;
    self.emit_body(body, "return", "")?;

    self.source.push_str("end");

    Ok(())
  }

  fn emit_if(&mut self, if_branches: Vec<IfBranch>, else_branch: Option<Vec<StatementNode>>) -> Result<(), Error> {
    let is_in_expression = self.get_scope().is_in_expression;
    self.source.push_str("if (");

    let last_branch = if_branches.len().checked_sub(1).unwrap_or(0);
    for (index, branch) in if_branches.into_iter().enumerate() {
      self.push_expression_scope();
      self.emit_expression(branch.condition)?;
      self.pop_scope();

      self.source.push_str(") then\n");

      if is_in_expression {
        self.emit_block(branch.body)?;
      } else {
        self.emit_body(branch.body, "return", "")?;
      }

      if index < last_branch {
        self.source.push_str("\nelseif (");
      }
    }

    if let Some(branch) = else_branch {
      self.source.push_str("\nelse\n");

      if is_in_expression {
        self.emit_block(branch)?;
      } else {
        self.emit_body(branch, "return", "")?;
      }
    } else if is_in_expression {
      self.source.push_str("\nelse nil\n");
    }

    if !is_in_expression {
      self.source.push_str("\nend\n");
    }

    Ok(())
  }

  fn emit_for(
    &mut self,
    lhs: Vec<LValueNode>,
    enumerable: ExpressionNode,
    body: Vec<StatementNode>,
  ) -> Result<(), Error> {
    let ret_symbol = format!("ret_{}", self.get_unique_id());
    let element_symbol = format!("element_{}", self.get_unique_id());
    let is_in_expression = self.get_scope().is_in_expression;

    if is_in_expression {
      write!(self.source, "(function()\nlocal {} = {{}}", ret_symbol).ok();
    }

    self.source.push_str("for ");

    self.push_lvalue_scope();
    self.emit_comma_separated(lhs, Self::emit_lvalue)?;
    let table_unpacks = std::mem::take(self.get_scope_mut().table_unpacks.as_mut());
    self.pop_scope();

    self.source.push_str(" in ");

    self.push_expression_scope();
    self.emit_expression(enumerable)?;
    self.pop_scope();

    self.source.push_str(" do \n");

    self.emit_table_unpacks(table_unpacks)?;
    // TODO: Jump to end of loop! Return won't actually return here
    self.emit_body(body, format!("local {} = ", element_symbol).as_str(), "")?;

    if is_in_expression {
      write!(self.source, "\ntable.insert({}, {})\n", ret_symbol, element_symbol).ok();
    }

    self.source.push_str("end");

    if is_in_expression {
      write!(self.source, "\nreturn {}\nend)()", ret_symbol).ok();
    }

    Ok(())
  }

  fn emit_call(&mut self, is_optional: bool, lhs: ExpressionNode, arguments: Vec<ExpressionNode>) -> Result<(), Error> {
    let is_direct_symbol_call = matches!(*lhs.value, Expression::Symbol(_));

    // Luau throws for `(<expr>)(<...args>)` if not explicitly an expression
    // while `<symbol>(<...args>)` works fine
    if !is_direct_symbol_call && !self.get_scope().is_in_expression {
      self.source.push_str("local _ = ");
    }

    self.push_expression_scope();

    if is_optional {
      self.source.push_str("(function() local lhs = ");
    }

    if is_direct_symbol_call {
      self.emit_expression(lhs)?;
    } else {
      self.source.push('(');
      self.emit_expression(lhs)?;
      self.source.push(')');
    }

    if is_optional {
      self.source.push_str("; return if lhs == nil then nil else lhs");
    }

    self.source.push('(');
    self.emit_comma_separated(arguments, Self::emit_expression)?;
    self.source.push(')');

    if is_optional {
      self.source.push_str(" end)()");
    }

    self.pop_scope();

    Ok(())
  }
}
