use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenValue::*;

impl Lexer {
  pub(in crate::lexer) fn tokenise_numeral(&mut self) -> Option<Token> {
    if !self.source.peek_next().is_some_and(|char| char.is_ascii_digit()) {
      return None;
    }

    let mut requires_digit_for: Option<&str> = None;

    let (supports_float, advance_by, radix) = match self.source.peek(1) {
      Some('x') => {
        requires_digit_for = Some("0x");
        (false, 2, 16)
      }
      Some('b') => {
        requires_digit_for = Some("0b");
        (false, 2, 2)
      }
      _ => (true, 0, 10),
    };

    self.advance_by(advance_by);

    let mut is_float = false;
    let mut numeral = "".to_string();

    while let Some(char) = self.source.peek_next()
      && char.is_digit(radix)
    {
      numeral.push(*char);
      self.advance();

      if self.source.is_match("_".chars()) {
        self.advance();
        requires_digit_for = Some("_");
      } else if self.source.is_match(".".chars()) {
        self.advance();

        if !supports_float {
          return Some(self.new_token(MalformedNumber(
            "Hex and binary number literals do not support floating point".to_string(),
          )));
        }

        if is_float {
          return Some(self.new_token(MalformedNumber(
            "Numbers must contain no more than one decimal point".to_string(),
          )));
        }

        is_float = true;
        requires_digit_for = Some(".");
        numeral.push('.');
      } else {
        requires_digit_for = None;
      }
    }

    if let Some(required_for) = requires_digit_for {
      return Some(self.new_token(MalformedNumber(format!("A digit must follow '{}'", required_for))));
    }

    if is_float {
      match numeral.parse::<f64>() {
        Ok(value) => Some(self.new_token(Double(value))),
        Err(err) => Some(self.new_token(MalformedNumber(err.to_string()))),
      }
    } else {
      match i64::from_str_radix(&numeral, radix) {
        Ok(value) => Some(self.new_token(Int(value))),
        Err(err) => Some(self.new_token(MalformedNumber(err.to_string()))),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use assertor::*;
  use moonjuice_common::Position;
  use parameterized::parameterized;

  #[parameterized(
    lexeme = { "123", "0xA4", "0b1101", "1_234" },
    value = { 123, 0xA4, 0b1101, 1234 }
  )]
  fn should_parse_integer_numeral(lexeme: &str, value: i64) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Int(value),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[parameterized(
    lexeme = { "1.23", "1_234.567_89" },
    value = { 1.23, 1234.56789 }
  )]
  fn should_parse_float_numeral(lexeme: &str, value: f64) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Double(value),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[parameterized(
    lexeme = { "0x", "0b", "1.", "1_" },
    error = { "0x", "0b", ".", "_" }
  )]
  fn should_handle_malformed_numbers_when_missing_following_digit(lexeme: &str, error: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedNumber(format!("A digit must follow '{}'", error)),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[parameterized(
    prefix = { "0x", "0b" }
  )]
  fn should_handle_malformed_numbers_when_hex_or_binary_is_float(prefix: &str) {
    let tokens = Lexer::tokenise(format!("{}1.2", prefix).chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedNumber("Hex and binary number literals do not support floating point".to_string()),
      lexeme: format!("{}1.2", prefix),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: prefix.len() + 4,
      },
    }]);
  }

  #[test]
  fn should_handle_malformed_numbers_when_multiple_decimal_points() {
    let tokens = Lexer::tokenise("1.2.3.4".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedNumber("Numbers must contain no more than one decimal point".to_string()),
      lexeme: "1.2.3.4".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 1, column: 8 },
    }]);
  }

  #[parameterized(
    lexeme = { "12FAB", "0b0123", "0b12AB", "1G", "0b1G", "0x1G" },
    error = {
      "Number contains hexadecimal digits, did you mean 0x12FAB?",
      "Number contains decimal digits, did you mean 0123?",
      "Number contains hexadecimal digits, did you mean 0x12AB?",
      "Number contains invalid characters",
      "Number contains invalid characters",
      "Number contains invalid characters"
    }
  )]
  fn should_handle_malformed_numbers_when_unsupported_digit_for_radix(lexeme: &str, error: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedNumber(error.to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }
}
