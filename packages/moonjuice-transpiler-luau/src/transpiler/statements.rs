use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::expression::ExpressionNode;
use moonjuice_parser::nodes::lvalue::LValueNode;
use moonjuice_parser::nodes::statement::Statement::{Break, Definition, Expression, Return, SyntaxError};
use moonjuice_parser::nodes::statement::StatementNode;
use std::fmt::Write;

impl LuauTranspiler {
  pub(super) fn emit_statement(
    &mut self,
    statement: StatementNode,
    should_return_expressions: bool,
    return_statement: &str,
    return_statement_post: &str,
  ) -> Result<(), Error> {
    let start = statement.start.clone();
    let end = statement.end.clone();

    match *statement.value {
      Definition {
        is_constant,
        is_export,
        lhs,
        rhs,
      } => self.emit_definition(is_constant, is_export, lhs, rhs)?,
      Return(expr) => self.emit_return(expr)?,
      Break => self.emit_break(),
      SyntaxError(message) => {
        return Err(Error { message, start, end });
      }
      Expression(expr) => {
        if should_return_expressions {
          write!(self.source, "{} ", return_statement).ok();
          self.push_expression_scope();
        }

        self.emit_expression(ExpressionNode {
          value: expr.into(),
          start,
          end,
        })?;

        if should_return_expressions {
          self.pop_scope();
          self.source.push_str(return_statement_post);
        }
      }
    }

    Ok(())
  }

  fn emit_definition(
    &mut self,
    is_constant: bool,
    is_export: bool,
    lhs: Vec<LValueNode>,
    rhs: Vec<ExpressionNode>,
  ) -> Result<(), Error> {
  }

  fn emit_return(&mut self, expr: ExpressionNode) -> Result<(), Error> {}

  fn emit_break(&mut self) {
    self.source.push_str("break");
  }
}
