#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_transpile_if_statement, "if true then false end");
  snapshot!(should_transpile_if_expression, "def x = if true then false end");

  snapshot!(should_collapse_single_expr_do_block, "def x = do 5 end");
  snapshot!(should_not_collapse_multi_expr_do_block, "def x = do 5 10 end");
  snapshot!(should_not_collapse_statement_do_block, "def x = do def y = 5 end");

  snapshot!(should_call_function_directly_when_lhs_symbol, "def x = fun()");
}
