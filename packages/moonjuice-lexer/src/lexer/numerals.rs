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
          return Some(self.new_token(MalformedNumber("More than one decimal point".to_string())));
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
}
