#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_call_function_directly_when_lhs_symbol, "fun()");

  snapshot!(should_call_function_indirectly_when_lhs_expression, "do fun end()");

  snapshot!(should_not_wrap_define, "def x = y");
  snapshot!(should_not_wrap_define_in_block, "do def x = y end");
  snapshot!(should_not_wrap_assignment, "x = y");
  snapshot!(should_not_wrap_assignment_in_block, "do x = y end");
}
