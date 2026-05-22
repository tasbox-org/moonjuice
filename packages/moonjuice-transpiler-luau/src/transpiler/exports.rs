use crate::LuauTranspiler;
use moonjuice_parser::nodes::lvalue::LValue::{Symbol, SyntaxError, TableUnpack};
use moonjuice_parser::nodes::lvalue::{LValueNode, TableUnpackElement};
use std::fmt::Write;

pub(super) fn gather_exports(node: &LValueNode) -> Vec<String> {
  let mut exports = vec![];
  let mut to_process = vec![node];

  while !to_process.is_empty() {
    match &*to_process.pop().unwrap().value {
      Symbol(symbol) => exports.push(symbol.clone()),
      TableUnpack { elements } => {
        for element in elements {
          if let TableUnpackElement::Valid { variable, .. } = element.value.as_ref() {
            to_process.push(variable);
          }
        }
      }
      SyntaxError(_) => (),
    }
  }

  exports
}

impl LuauTranspiler {
  pub(super) fn emit_exports(&mut self, exports: Vec<String>) {
    let exports_symbol = self.get_scope().exports_symbol.clone();

    for export in exports {
      write!(self.source, "\n{}.{} = {}", exports_symbol, export, export).ok();
    }
  }
}
