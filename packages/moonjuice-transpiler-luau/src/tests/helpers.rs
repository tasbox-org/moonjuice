#[cfg(test)]
#[macro_export]
macro_rules! snapshot {
  ( $name:ident, $code:expr ) => {
    #[test]
    fn $name() {
      let result = $crate::transpile_to_luau($code.to_string(), "test.mj".to_string());

      insta::with_settings!({ description => $code }, {
        match result {
          Ok(luau_source) => insta::assert_snapshot!(luau_source),
          Err(error) => insta::assert_snapshot!(error.to_string())
        }
      });
    }
  };
}
