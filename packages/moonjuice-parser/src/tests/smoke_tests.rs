#[cfg(test)]
mod tests {
  use crate::snapshot;
  use indoc::indoc;

  snapshot!(
    should_parse_double_example,
    indoc! {"
      def double = fn(value) value * 2 end

      5 |> double |> print
    "}
  );
}
