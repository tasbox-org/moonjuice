use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::statement::Statement::Definition;
use moonjuice_parser::nodes::statement::StatementNode;

impl LuauTranspiler {
  pub(super) fn emit_table_unpacks(&mut self, table_unpacks: Vec<StatementNode>) -> Result<(), Error> {
    for unpack in table_unpacks {
      match *unpack.value {
        Definition {
          is_constant,
          is_export,
          lhs,
          rhs,
        } => {
          self.emit_definition(is_constant, is_export, lhs, rhs)?;
        }
        _ => {
          return Err(Error {
            message: "Unexpected AST node in table unpacks. Expected Definition (this should never happen!)"
              .to_string(),
            start: unpack.start,
            end: unpack.end,
          });
        }
      }
    }

    Ok(())
  }
}
