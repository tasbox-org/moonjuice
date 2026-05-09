use crate::{Error, LuauTranspiler};
use moonjuice_common::Position;
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
    if self.get_scope().is_in_expression || self.get_scope().is_in_lvalue {
      self.source.push_str(value);
    }
  }

  pub(super) fn emit_string(
    &mut self,
    segments: Vec<StringSegment>,
    arguments: Vec<ExpressionNode>,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    if !self.get_scope().is_in_expression {
      if arguments.is_empty() {
        return Ok(());
      } else {
        self.source.push_str("local _ = ");
      }
    }

    let mut arguments = arguments.into_iter();
    let last_segment = segments.len().checked_sub(1).unwrap_or(0);

    self.push_expression_scope();
    self.source.push('`');

    for (index, segment) in segments.into_iter().enumerate() {
      match segment {
        StringSegment::Valid(value) => {
          self.source.push_str(
            value
              .replace('\\', "\\\\")
              .replace('\r', "\\r")
              .replace('\n', "\\n")
              .replace('\t', "\\t")
              .replace('\0', "\\0")
              .replace('`', "\\`")
              .replace('{', "\\{")
              .as_str(),
          );
        }
        StringSegment::Malformed(message) => return Err(Error { message, start, end }),
      }

      if index < last_segment
        && let Some(argument) = arguments.next()
      {
        self.source.push('{');
        self.emit_expression(argument)?;
        self.source.push('}');
      }
    }

    self.source.push('`');
    self.pop_scope();

    Ok(())
  }
}
