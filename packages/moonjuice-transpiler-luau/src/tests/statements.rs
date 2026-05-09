#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_call_function_directly_when_lhs_symbol, "fun()");

  snapshot!(should_call_function_indirectly_when_lhs_expression, "do fun end()");
}
