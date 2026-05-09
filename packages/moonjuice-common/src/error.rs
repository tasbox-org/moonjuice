use crate::Position;
use std::fmt::{Display, Formatter};

pub struct Error {
  pub message: String,
  pub start: Position,
  pub end: Position,
  pub path: String,
  pub line_contents: String,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let line = self.start.line;
    let column = self.start.column;
    let end_column = self.end.column;
    let padding = " ".repeat(line.to_string().len());

    write!(
      f,
      "{}:{}:{}\n{} |\n{} | {}\n{} | {}{}\nSyntax Error: {}",
      self.path,
      line,
      column,
      padding,
      line,
      self.line_contents,
      padding,
      " ".repeat(column.checked_sub(1).unwrap_or(0)),
      "^".repeat(end_column.checked_sub(column).unwrap_or(1)),
      self.message
    )
  }
}
