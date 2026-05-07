use crate::Token;
use crate::TokenValue::{Double, Int, MalformedNumber};
use crate::lexer::Lexer;

impl Lexer {
  pub(in crate::lexer) fn tokenise_numeral(&mut self) -> Option<Token> {
    if !self.source.peek_next().is_some_and(|char| char.is_ascii_digit()) {
      return None;
    }

    let mut requires_digit_for: Option<&str> = None;

    let (supports_float, prefix, allowed_radix) = match self.source.peek(1) {
      Some('x') => {
        requires_digit_for = Some("0x");
        (false, "0x", 16)
      }
      Some('b') => {
        requires_digit_for = Some("0b");
        (false, "0b", 2)
      }
      _ => (true, "", 10),
    };

    self.advance_by(prefix.len());

    let mut num_decimal_points = 0;
    let mut max_radix = 0;
    let mut contains_invalid = false;
    let mut numeral = "".to_string();

    while let Some(char) = self.source.peek_next().cloned()
      && char.is_ascii_alphanumeric()
    {
      numeral.push(char);
      self.advance();

      if char.is_digit(2) {
        max_radix = std::cmp::max(2, max_radix);
      } else if char.is_digit(10) {
        max_radix = std::cmp::max(10, max_radix);
      } else if char.is_digit(16) {
        max_radix = std::cmp::max(16, max_radix);
      } else {
        contains_invalid = true;
      }

      if self.source.is_match("_".chars()) {
        self.advance();
        requires_digit_for = Some("_");
      } else if self.source.is_match(".".chars()) && !self.source.is_match("..".chars()) {
        self.advance();

        num_decimal_points += 1;
        requires_digit_for = Some(".");
        numeral.push('.');
      } else {
        requires_digit_for = None;
      }
    }

    if contains_invalid {
      Some(self.new_token(MalformedNumber(format!(
        "Number '{}{}' contains invalid characters",
        prefix, numeral
      ))))
    } else if max_radix > allowed_radix {
      match max_radix {
        16 => Some(self.new_token(MalformedNumber(format!(
          "Number contains hexadecimal digits, did you mean 0x{}?",
          numeral
        )))),
        10 => Some(self.new_token(MalformedNumber(format!(
          "Number contains decimal digits, did you mean {}?",
          numeral
        )))),
        _ => Some(self.new_token(MalformedNumber(format!("Number '{}' uses unsupported radix", numeral)))),
      }
    } else if let Some(required_for) = requires_digit_for {
      Some(self.new_token(MalformedNumber(format!("A digit must follow '{}'", required_for))))
    } else if num_decimal_points > 0 && !supports_float {
      Some(self.new_token(MalformedNumber(
        "Hex and binary number literals do not support floating point".to_string(),
      )))
    } else if num_decimal_points > 1 {
      Some(self.new_token(MalformedNumber(
        "Numbers must contain no more than one decimal point".to_string(),
      )))
    } else {
      if num_decimal_points > 0 {
        match numeral.parse::<f64>() {
          Ok(value) => Some(self.new_token(Double(value))),
          Err(err) => Some(self.new_token(MalformedNumber(err.to_string()))),
        }
      } else {
        match i64::from_str_radix(&numeral, allowed_radix) {
          Ok(value) => Some(self.new_token(Int(value))),
          Err(err) => Some(self.new_token(MalformedNumber(err.to_string()))),
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Operator::Concat;
  use crate::Token;
  use crate::TokenValue::{Double, MalformedNumber, Operator};
  use assertor::*;
  use moonjuice_common::Position;
  use rstest::rstest;

  #[rstest]
  #[case("123", 123)]
  #[case("0xA4", 0xA4)]
  #[case("0xbc", 0xBC)]
  #[case("0b1101", 0b1101)]
  #[case("1_234", 1234)]
  fn should_parse_integer_numeral(#[case] lexeme: &str, #[case] value: i64) {
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

  #[rstest]
  #[case("1.23", 1.23)]
  #[case("1_234.567_89", 1234.56789)]
  fn should_parse_float_numeral(#[case] lexeme: &str, #[case] value: f64) {
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

  #[rstest]
  #[case("0x", "0x")]
  #[case("0b", "0b")]
  #[case("1.", ".")]
  #[case("1_", "_")]
  fn should_handle_malformed_numbers_when_missing_following_digit(#[case] lexeme: &str, #[case] error: &str) {
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

  #[rstest]
  #[case("0x")]
  #[case("0b")]
  fn should_handle_malformed_numbers_when_hex_or_binary_is_float(#[case] prefix: &str) {
    let tokens = Lexer::tokenise(format!("{}1.1", prefix).chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: MalformedNumber("Hex and binary number literals do not support floating point".to_string()),
      lexeme: format!("{}1.1", prefix),
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

  #[rstest]
  #[case("12FAB", "Number contains hexadecimal digits, did you mean 0x12FAB?")]
  #[case("0b0123", "Number contains decimal digits, did you mean 0123?")]
  #[case("0b12AB", "Number contains hexadecimal digits, did you mean 0x12AB?")]
  #[case("1G", "Number '1G' contains invalid characters")]
  #[case("0b1G", "Number '0b1G' contains invalid characters")]
  #[case("0x1G", "Number '0x1G' contains invalid characters")]
  fn should_handle_malformed_numbers_when_unsupported_digit_for_radix(#[case] lexeme: &str, #[case] error: &str) {
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

  #[test]
  fn should_not_consume_concat_operator() {
    let tokens = Lexer::tokenise("1..2".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![
      Token {
        value: Int(1),
        lexeme: "1".to_string(),
        start: Position { line: 1, column: 1 },
        end: Position { line: 1, column: 2 },
      },
      Token {
        value: Operator(Concat),
        lexeme: "..".to_string(),
        start: Position { line: 1, column: 2 },
        end: Position { line: 1, column: 4 },
      },
      Token {
        value: Int(2),
        lexeme: "2".to_string(),
        start: Position { line: 1, column: 4 },
        end: Position { line: 1, column: 5 },
      },
    ])
  }
}
