use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::statement::StatementNode;

impl LuauTranspiler {
  pub(super) fn emit_table_unpacks(&mut self, table_unpacks: Vec<StatementNode>) -> Result<(), Error> {}
}
