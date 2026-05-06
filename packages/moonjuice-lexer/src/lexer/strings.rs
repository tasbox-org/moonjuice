use crate::TokenValue::MalformedString;
use crate::lexer::Lexer;
use crate::{Token, TokenValue};
use std::iter;
use std::string::String;

impl Lexer {
  pub(in crate::lexer) fn tokenise_string(&mut self) -> Option<Token> {
    let delimiter = self.consume_string_delimiter()?;

    let mut string = "".to_string();

    while let Some(char) = self.source.peek_next().cloned()
      && !self.source.is_match(delimiter.chars())
    {
      string.push(char);
      self.advance();
    }

    if self.source.is_match(delimiter.chars()) {
      self.advance_by(delimiter.len());
      Some(self.new_token(TokenValue::String(string)))
    } else {
      Some(self.new_token(MalformedString(format!("Missing closing {}", delimiter))))
    }
  }

  fn consume_string_delimiter(&mut self) -> Option<String> {
    let delimiter_char = *self.source.peek_next()?;
    if delimiter_char != '"' && delimiter_char != '\'' {
      return None;
    }

    self.advance();
    let mut delimiter_length = 1;

    while self.source.peek_next().is_some_and(|char| *char == delimiter_char) {
      self.advance();
      delimiter_length += 1;
    }

    Some(iter::repeat(delimiter_char).take(delimiter_length).collect())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Token;
  use crate::TokenValue::{FormatStringEnd, FormatStringMiddle, FormatStringStart};
  use assertor::*;
  use indoc::indoc;
  use moonjuice_common::Position;
  use rstest::rstest;

  #[rstest]
  #[case("'")]
  #[case("\"")]
  #[case("''")]
  #[case("\"\"")]
  fn should_parse_single_line_string(#[case] delimiter: &str) {
    let string = "some string";
    let lexeme = format!("{}{}{}", delimiter, string, delimiter);
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String(string.to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[rstest]
  #[case("'")]
  #[case("\"")]
  #[case("''")]
  #[case("\"\"")]
  fn should_parse_multiline_string(#[case] delimiter: &str) {
    let lexeme = format!(
      indoc! {"
        {}
        multi-line string
        unindented
        {}
      "},
      delimiter, delimiter
    )
    .trim_end()
    .to_string();
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String("multi-line string\nunindented".to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 2 },
    }]);
  }

  #[rstest]
  #[case(" ")]
  #[case("  ")]
  #[case("    ")]
  #[case("\t")]
  fn should_parse_multiline_string_with_unindent(#[case] indentation: &str) {
    let lexeme = format!(
      indoc! {"
        '
        {}unindented
        {}{}indented
        '
      "},
      indentation, indentation, indentation
    );
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String("unindented\n  indented".to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 2 },
    }]);
  }

  #[rstest]
  #[case("\\\"", "\"")]
  #[case("\\'", "'")]
  #[case("\\n", "\n")]
  #[case("\\r", "\r")]
  #[case("\\t", "\t")]
  #[case("\\\\", "\\")]
  #[case("\\0", "\0")]
  #[case("\\x41", "\x41")]
  #[case("\\u{263A}", "☺")]
  #[case("\\{}", "{}")]
  fn should_parse_escape_sequences(#[case] sequence: &str, #[case] value: &str) {
    let lexeme = format!("'{}'", sequence,);
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String(value.to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[test]
  fn should_parse_format_string() {
    let tokens = Lexer::tokenise("'start{}middle{}end'".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![
      Token {
        value: FormatStringStart("start".to_string()),
        lexeme: "'start".to_string(),
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 7 },
      },
      Token {
        value: FormatStringMiddle("middle".to_string()),
        lexeme: "middle".to_string(),
        start: Position { line: 1, column: 9 },
        end: Position { line: 1, column: 14 },
      },
      Token {
        value: FormatStringEnd("end".to_string()),
        lexeme: "end'".to_string(),
        start: Position { line: 1, column: 16 },
        end: Position { line: 1, column: 21 },
      },
    ]);
  }

  #[test]
  fn should_parse_minimal_format_string() {
    let tokens = Lexer::tokenise("'{}'".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![
      Token {
        value: FormatStringStart("".to_string()),
        lexeme: "'".to_string(),
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 2 },
      },
      Token {
        value: FormatStringEnd("end".to_string()),
        lexeme: "'".to_string(),
        start: Position { line: 1, column: 4 },
        end: Position { line: 1, column: 5 },
      },
    ]);
  }

  #[rstest]
  #[case("'a", "'")]
  #[case("\"b", "\"")]
  #[case("''mismatched'", "''")]
  fn should_return_malformed_string_when_missing_terminator(#[case] lexeme: &str, #[case] delimiter: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedString(format!("Missing closing {}", delimiter).to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }
}
