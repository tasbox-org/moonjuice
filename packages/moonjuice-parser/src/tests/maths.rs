#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_parse_unary_operation, "-5");
  snapshot!(should_parse_binary_operation, "1 + 2");
  snapshot!(should_parse_left_associative, "1 + 2 + 3");
  snapshot!(should_parse_right_associative, "1 ?? 2 ?? 3");
}
