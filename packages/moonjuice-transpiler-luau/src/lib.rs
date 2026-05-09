use std::fmt::Write;
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

pub fn tokenise_parse_and_transpile(source: Vec<char>) -> Result<String, Error> {
  let tokens = Lexer::tokenise(source);
  let ast = Parser::parse(tokens);

  LuauTranspiler::transpile(ast)
}
