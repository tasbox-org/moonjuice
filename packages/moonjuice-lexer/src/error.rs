#[derive(Debug)]
pub enum Error {
  UnterminatedMultilineComment,
  UnexpectedCharacter(char),
}
