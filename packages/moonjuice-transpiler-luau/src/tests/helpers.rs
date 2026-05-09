#[cfg(test)]
#[macro_export]
macro_rules! snapshot {
  ( $name:ident, $code:expr ) => {
    #[test]
    fn $name() {
      let result = $crate::transpile_to_luau($code.to_string(), "test.mj".to_string());
      let formatted = yaml_serde::to_string(&result).unwrap();

      insta::with_settings!({ description => $code }, {
        insta::assert_snapshot!(formatted);
      });
    }
  };
}
