#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_parse_def, "def a = b");
  snapshot!(should_parse_mut, "mut a = b");
  snapshot!(should_parse_multiple_assignment, "def a, b = c, d");
  snapshot!(should_parse_multiple_assignment_when_more_declarations, "def a, b = c");
  snapshot!(should_parse_multiple_assignment_when_more_assignments, "def a = b, c");

  snapshot!(should_parse_empty_unpack, "def {} = a");
  snapshot!(should_parse_explicit_unpack, "def { [1] = b } = a");
  snapshot!(should_parse_index_shorthand_unpack, "def { b } = a");
  snapshot!(should_parse_key_shorthand_unpack, "def { .b } = a");
  snapshot!(should_parse_aliased_key_shorthand_unpack, "def { .b = c } = a");

  snapshot!(should_parse_return, "return a");
  snapshot!(should_parse_break, "break");

  snapshot!(should_parse_export, "export a = b");
}
