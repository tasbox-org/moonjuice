#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_parse_unary_operation, "-5");
}
