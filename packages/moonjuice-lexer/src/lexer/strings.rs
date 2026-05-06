use crate::StringTokenType::Whole;
use crate::TokenValue::MalformedString;
use crate::lexer::Lexer;
use crate::{Token, TokenValue};
use std::iter;
use std::string::String;

impl Lexer {
  pub(in crate::lexer) fn tokenise_string(&mut self) -> Option<Vec<Token>> {
    let delimiter = self.consume_string_delimiter()?;

    // let mut tokens = vec![];

    let mut string = "".to_string();
    let mut contains_invalid_escapes = false;

    while let Some(char) = self.source.peek_next().cloned()
      && !self.source.is_match(delimiter.chars())
    {
      self.advance();

      match char {
        '\\' => {
          if let Some(escape_sequence) = self.consume_escape_sequence() {
            string += escape_sequence.as_str();
          } else {
            contains_invalid_escapes = true;
          }
        }
        _ => {
          string.push(char);
        }
      }
    }

    if !self.source.is_match(delimiter.chars()) {
      Some(vec![self.new_token(MalformedString(
        Whole,
        format!("Missing closing {}", delimiter),
      ))])
    } else if contains_invalid_escapes {
      Some(vec![
        self.new_token(MalformedString(Whole, "Invalid escape sequence".to_string())),
      ])
    } else {
      self.advance_by(delimiter.len());
      Some(vec![self.new_token(TokenValue::String(Whole, string))])
    }
  }

  fn consume_string_delimiter(&mut self) -> Option<String> {
    let delimiter_char = *self.source.peek_next()?;
    if delimiter_char != '"' && delimiter_char != '\'' {
      return None;
    }

    self.advance();
    let mut delimiter_length = 1;

    while self.source.peek_next().is_some_and(|char| *char == delimiter_char)
      && (delimiter_length > 1 || self.source.peek(1).is_some_and(|char| *char == delimiter_char))
    {
      self.advance();
      delimiter_length += 1;
    }

    Some(iter::repeat(delimiter_char).take(delimiter_length).collect())
  }

  fn consume_escape_sequence(&mut self) -> Option<String> {
    let (value, advance_by) = match self.source.peek_next().cloned() {
      Some('"') => Some(("\"".to_string(), 1)),
      Some('\'') => Some(("\'".to_string(), 1)),
      Some('n') => Some(("\n".to_string(), 1)),
      Some('r') => Some(("\r".to_string(), 1)),
      Some('t') => Some(("\t".to_string(), 1)),
      Some('\\') => Some(("\\".to_string(), 1)),
      Some('0') => Some(("\0".to_string(), 1)),
      Some('{') => Some(("{".to_string(), 1)),
      Some('\n') => Some(("".to_string(), 1)),
      Some('x') => {
        if let Some(first) = self.source.peek(1)
          && let Some(second) = self.source.peek(2)
          && first.is_ascii_hexdigit()
          && second.is_ascii_hexdigit()
          && let Ok(value) = u8::from_str_radix(format!("{}{}", first, second).as_str(), 16)
          && value <= 0x7F
        {
          Some(((value as char).to_string(), 3))
        } else {
          None
        }
      }
      _ => None,
    }?;

    self.advance_by(advance_by);

    Some(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::StringTokenType::{End, Middle, Start};
  use crate::Token;
  use assertor::*;
  use indoc::indoc;
  use moonjuice_common::Position;
  use rstest::rstest;

  #[rstest]
  #[case("'")]
  #[case("\"")]
  #[case("'''")]
  #[case("\"\"\"")]
  #[case("''''")]
  #[case("\"\"\"\"")]
  fn should_parse_single_line_string(#[case] delimiter: &str) {
    let string = "some string";
    let lexeme = format!("{}{}{}", delimiter, string, delimiter);
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String(Whole, string.to_string()),
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
  #[case("'''")]
  #[case("\"\"\"")]
  #[case("''''")]
  #[case("\"\"\"\"")]
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
      value: TokenValue::String(Whole, "\nmulti-line string\nunindented\n".to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 4,
        column: delimiter.len() + 1,
      },
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
      value: TokenValue::String(Whole, "\nunindented\n  indented\n".to_string()),
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
      value: TokenValue::String(Whole, value.to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[test]
  fn should_parse_escaped_newline() {
    let lexeme = "'\\\n'";
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String(Whole, "".to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 2, column: 2 },
    }]);
  }

  #[rstest]
  #[case("\"\"")]
  #[case("''")]
  fn should_parse_empty_string(#[case] lexeme: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::String(Whole, "".to_string()),
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
        value: TokenValue::String(Start, "start".to_string()),
        lexeme: "'start".to_string(),
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 7 },
      },
      Token {
        value: TokenValue::String(Middle, "middle".to_string()),
        lexeme: "middle".to_string(),
        start: Position { line: 1, column: 9 },
        end: Position { line: 1, column: 14 },
      },
      Token {
        value: TokenValue::String(End, "end".to_string()),
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
        value: TokenValue::String(Start, "".to_string()),
        lexeme: "'".to_string(),
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 2 },
      },
      Token {
        value: TokenValue::String(End, "end".to_string()),
        lexeme: "'".to_string(),
        start: Position { line: 1, column: 4 },
        end: Position { line: 1, column: 5 },
      },
    ]);
  }

  #[rstest]
  #[case("'a", "'")]
  #[case("\"b", "\"")]
  #[case("'''mismatched''", "'''")]
  fn should_return_malformed_string_when_missing_terminator(#[case] lexeme: &str, #[case] delimiter: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedString(Whole, format!("Missing closing {}", delimiter).to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }
}
