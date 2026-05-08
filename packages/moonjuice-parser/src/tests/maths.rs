#[cfg(test)]
mod tests {
  use crate::tokenise_and_parse;

  #[test]
  fn should_parse_unary_operation() {
    insta::assert_yaml_snapshot!(tokenise_and_parse("-5".chars().collect()))
  }
}
