#[cfg(test)]
#[macro_export]
macro_rules! snapshot {
  ( $name:ident, $code:expr ) => {
    #[test]
    fn $name() {
      let result = $crate::tokenise_parse_and_transpile($code.chars().collect());
      let formatted = yaml_serde::to_string(&result).unwrap();

      insta::with_settings!({ description => $code }, {
        insta::assert_snapshot!(formatted);
      });
    }
  };
}
