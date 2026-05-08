mod blocks;
mod expressions;
mod lvalues;
mod primitives;
mod scopes;
mod statements;

use crate::LuauTranspiler;

impl LuauTranspiler {
  pub(crate) fn new() -> Self {
    LuauTranspiler {
      scopes: vec![],
      source: String::new(),
    }
  }
}
