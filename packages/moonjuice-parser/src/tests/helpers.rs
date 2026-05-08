#[cfg(test)]
#[macro_export]
macro_rules! snapshot {
  ( $name:ident, $code:expr ) => {
    #[test]
    fn $name() {
      insta::assert_yaml_snapshot!($crate::tokenise_and_parse($code.chars().collect()))
    }
  };
}
