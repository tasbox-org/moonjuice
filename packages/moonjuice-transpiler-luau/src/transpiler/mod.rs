mod blocks;
mod expressions;
mod lvalues;
mod primitives;
mod scopes;
mod statements;
mod table_unpacks;

use crate::{Error, LuauTranspiler};

impl LuauTranspiler {
  pub(crate) fn new() -> Self {
    LuauTranspiler {
      scopes: vec![],
      source: String::new(),
    }
  }

  pub(crate) fn emit_comma_separated<T>(
    &mut self,
    elements: Vec<T>,
    callback: impl Fn(&mut Self, T) -> Result<(), Error>,
  ) -> Result<(), Error> {
    let last = elements.len().checked_sub(1).unwrap_or(0);

    for (index, element) in elements.into_iter().enumerate() {
      callback(self, element)?;

      if index < last {
        self.source.push_str(", ");
      }
    }

    Ok(())
  }
}
