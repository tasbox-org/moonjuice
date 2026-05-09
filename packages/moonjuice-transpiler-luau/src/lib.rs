use std::fmt::{Display, Formatter, Write};
mod tests;
pub mod transpiler;

use moonjuice_common::Position;
use moonjuice_lexer::Lexer;
use moonjuice_parser::Parser;
use moonjuice_parser::nodes::statement::StatementNode;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Error {
  pub message: String,
  pub start: Position,
  pub end: Position,
}

#[derive(Serialize)]
pub struct EnrichedError {
  pub error: Error,
  pub path: String,
  pub line_contents: String,
}

impl Display for EnrichedError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let line = self.error.start.line;
    let column = self.error.start.column;
    let end_column = self.error.end.column;
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
      self.error.message
    )
  }
}

pub(crate) struct Scope {
  pub is_in_expression: bool,
  pub is_in_lvalue: bool,

  pub exports_symbol: String,
  pub table_unpacks: Vec<StatementNode>,
}

pub struct LuauTranspiler {
  scopes: Vec<Scope>,
  source: String,
}

impl LuauTranspiler {
  pub fn transpile(ast: Vec<StatementNode>) -> Result<String, Error> {
    let mut transpiler = LuauTranspiler::new();

    let exports_symbol = format!("exports_{}", Uuid::now_v7().simple());
    write!(transpiler.source, "local {} = {{}}\n\n", exports_symbol).ok();

    transpiler.push_root_scope(exports_symbol.clone());
    transpiler.emit_body(ast, "return", "")?;
    transpiler.pop_scope();

    write!(transpiler.source, "\nreturn {}", exports_symbol).ok();
    Ok(transpiler.source)
  }
}

pub fn transpile_to_luau(source: String, path: String) -> Result<String, EnrichedError> {
  let tokens = Lexer::tokenise(source.chars().collect());
  let ast = Parser::parse(tokens);

  LuauTranspiler::transpile(ast).map_err(|error| {
    let lines: Vec<&str> = source.split("\n").collect();
    let line = lines
      .get(error.start.line.checked_sub(1).unwrap_or(0))
      .map(|line| *line)
      .unwrap_or("");

    EnrichedError {
      error,
      path,
      line_contents: line.to_string(),
    }
  })
}

#[cxx::bridge(namespace = "MoonJuice")]
mod ffi {
  extern "Rust" {
    type Error;

    fn transpile_to_luau(source: String, path: String) -> Result<String>;
  }
}
