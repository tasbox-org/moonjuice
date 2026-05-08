use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::statement::StatementNode;

impl LuauTranspiler {
  pub(super) fn emit_block(&mut self, body: Vec<StatementNode>) -> Result<(), Error> {
    let is_in_expression = self.get_scope().is_in_expression;

    if is_in_expression {
      self.source.push_str("(function() ");
    } else {
      self.source.push_str("do\n");
    }

    self.emit_body(body, "return", "")?;

    if is_in_expression {
      self.source.push_str("end)()");
    } else {
      self.source.push_str("end\n");
    }

    Ok(())
  }

  pub(crate) fn emit_body(
    &mut self,
    body: Vec<StatementNode>,
    return_statement: &str,
    return_statement_post: &str,
  ) -> Result<(), Error> {
    let is_in_expression = self.get_scope().is_in_expression;

    self.push_statement_scope();

    let last = body.len().checked_sub(1).unwrap_or(0);
    for (index, node) in body.into_iter().enumerate() {
      let should_return = index == last && is_in_expression;

      self.emit_statement(node, should_return, return_statement, return_statement_post)?;
      self.source.push('\n');
    }

    self.pop_scope();

    Ok(())
  }
}
