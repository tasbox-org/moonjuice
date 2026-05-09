#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_return_error_when_assignment_lhs_not_lvalue, "def 5 = 6");
}
