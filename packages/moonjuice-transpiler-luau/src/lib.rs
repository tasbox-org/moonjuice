use std::fmt::Write;
mod tests;
pub mod transpiler;

use moonjuice_common::Position;
use moonjuice_lexer::Lexer;
use moonjuice_parser::Parser;
use moonjuice_parser::nodes::statement::StatementNode;

#[cfg(not(test))]
use uuid::Uuid;

pub struct Error {
  pub message: String,
  pub start: Position,
  pub end: Position,
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

  #[cfg(test)]
  next_unique_id: u64,
}

impl LuauTranspiler {
  pub fn transpile(ast: Vec<StatementNode>) -> Result<String, Error> {
    let mut transpiler = LuauTranspiler::new();

    let exports_symbol = format!("exports_{}", transpiler.get_unique_id());
    write!(transpiler.source, "local {} = {{}}\n\n", exports_symbol).ok();

    transpiler.push_root_scope(exports_symbol.clone());
    transpiler.emit_body(ast, "return", "")?;
    transpiler.pop_scope();

    write!(transpiler.source, "\nreturn {}", exports_symbol).ok();
    Ok(transpiler.source)
  }

  #[cfg(test)]
  pub(crate) fn get_unique_id(&mut self) -> String {
    self.next_unique_id += 1;
    format!("TEST_ID_{}", self.next_unique_id)
  }

  #[cfg(not(test))]
  pub(crate) fn get_unique_id(&self) -> String {
    Uuid::now_v7().simple().to_string()
  }
}

pub fn transpile_to_luau(source: String, path: String) -> Result<String, moonjuice_common::error::Error> {
  let tokens = Lexer::tokenise(source.chars().collect());
  let ast = Parser::parse(tokens);

  LuauTranspiler::transpile(ast).map_err(|error| {
    let lines: Vec<&str> = source.split("\n").collect();
    let line = lines
      .get(error.start.line.checked_sub(1).unwrap_or(0))
      .map(|line| *line)
      .unwrap_or("");

    moonjuice_common::error::Error {
      message: error.message,
      start: error.start,
      end: error.end,
      path,
      line_contents: line.to_string(),
    }
  })
}
