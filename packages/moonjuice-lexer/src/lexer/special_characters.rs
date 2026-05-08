use crate::Token;
use crate::TokenValue::SpecialCharacter;
use crate::lexer::Lexer;
use moonjuice_common::SpecialCharacter::{
  CloseBracket, CloseCurlyBracket, CloseSquareBracket, Colon, Comma, OpenBracket, OpenCurlyBracket, OpenSquareBracket,
};

impl Lexer {
  pub(super) fn tokenise_special_character(&mut self) -> Option<Token> {
    let value = match self.source.peek_next() {
      Some('(') => Some(OpenBracket),
      Some(')') => Some(CloseBracket),
      Some('[') => Some(OpenSquareBracket),
      Some(']') => Some(CloseSquareBracket),
      Some('{') => Some(OpenCurlyBracket),
      Some('}') => Some(CloseCurlyBracket),
      Some(',') => Some(Comma),
      Some(':') => Some(Colon),
      _ => None,
    }?;

    self.advance();

    Some(self.new_token(SpecialCharacter(value)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Token;
  use assertor::*;
  use moonjuice_common::Position;
  use rstest::rstest;

  #[rstest]
  #[case('(', OpenBracket)]
  #[case(')', CloseBracket)]
  #[case('[', OpenSquareBracket)]
  #[case(']', CloseSquareBracket)]
  #[case('{', OpenCurlyBracket)]
  #[case('}', CloseCurlyBracket)]
  #[case(',', Comma)]
  #[case(':', Colon)]
  fn should_parse_special_character(#[case] character: char, #[case] expected: moonjuice_common::SpecialCharacter) {
    let tokens = Lexer::tokenise(vec![character]);

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: SpecialCharacter(expected),
      lexeme: character.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 1, column: 2 },
    }]);
  }
}
