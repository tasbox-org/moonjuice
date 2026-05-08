use crate::{Error, LuauTranspiler};
use moonjuice_parser::nodes::lvalue::{LValue, LValueNode, TableUnpackElement};

impl LuauTranspiler {
  pub(super) fn emit_lvalue(&mut self, lvalue: LValueNode) -> Result<(), Error> {
    let start = lvalue.start;
    let end = lvalue.end;

    match *lvalue.value {
      LValue::Symbol(value) => self.emit_symbol(value.as_str()),
      LValue::TableUnpack { elements } => self.emit_table_unpack(elements)?,
      LValue::SyntaxError(message) => return Err(Error { message, start, end }),
    }

    Ok(())
  }

  fn emit_table_unpack(&mut self, elements: Vec<TableUnpackElement>) -> Result<(), Error> {}
}
