use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::expression::{ExpressionNode, StringSegment};
use std::fmt::Write;

impl LuauTranspiler {
  pub(super) fn emit_nil(&mut self) {
    if self.get_scope().is_in_expression {
      self.source.push_str("nil");
    }
  }

  pub(super) fn emit_bool(&mut self, value: bool) {
    if self.get_scope().is_in_expression {
      self.source.push_str(if value { "true" } else { "false" });
    }
  }

  pub(super) fn emit_int(&mut self, value: i64) {
    if self.get_scope().is_in_expression {
      write!(self.source, "{}", value).ok();
    }
  }

  pub(super) fn emit_double(&mut self, value: f64) {
    if self.get_scope().is_in_expression {
      write!(self.source, "{}", value).ok();
    }
  }

  pub(super) fn emit_symbol(&mut self, value: &str) {
    if self.get_scope().is_in_expression {
      self.source.push_str(value);
    }
  }

  pub(super) fn emit_string(
    &mut self,
    segments: Vec<StringSegment>,
    arguments: Vec<ExpressionNode>,
  ) -> Result<(), Error> {
  }
}
