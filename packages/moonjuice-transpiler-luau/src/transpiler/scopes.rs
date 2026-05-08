use crate::{LuauTranspiler, Scope};

impl LuauTranspiler {
  pub(crate) fn get_scope(&self) -> &Scope {
    self.scopes.last().unwrap()
  }

  pub(crate) fn push_expression_scope(&mut self) {
    let current_scope = self.get_scope();

    self.scopes.push(Scope {
      is_in_expression: true,
      is_in_lvalue: false,
      exports_symbol: current_scope.exports_symbol.clone(),
      table_unpacks: vec![],
    });
  }

  pub(crate) fn push_lvalue_scope(&mut self) {
    let current_scope = self.get_scope();

    self.scopes.push(Scope {
      is_in_expression: false,
      is_in_lvalue: true,
      exports_symbol: current_scope.exports_symbol.clone(),
      table_unpacks: vec![],
    });
  }

  pub(crate) fn push_statement_scope(&mut self) {
    let current_scope = self.get_scope();

    self.scopes.push(Scope {
      is_in_expression: false,
      is_in_lvalue: false,
      exports_symbol: current_scope.exports_symbol.clone(),
      table_unpacks: vec![],
    });
  }

  pub(crate) fn push_root_scope(&mut self, exports_symbol: String) {
    self.scopes.push(Scope {
      is_in_expression: false,
      is_in_lvalue: false,
      exports_symbol,
      table_unpacks: vec![],
    });
  }

  pub(crate) fn pop_scope(&mut self) {
    self.scopes.pop();
  }
}
