use crate::Operator::*;
use crate::Token;
use crate::TokenValue;
use crate::lexer::Lexer;

impl Lexer {
  pub(super) fn tokenise_operator(&mut self) -> Option<Token> {
    let char = self.source.peek_next()?;

    let (operator, advance_by) = match char {
      '+' => Some((Add, 1)),
      '-' => Some((Subtract, 1)),
      '*' => Some((Multiply, 1)),
      '/' => Some((Divide, 1)),
      '%' => Some((Modulo, 1)),
      '=' => {
        if self.source.is_match("==".chars()) {
          Some((Equals, 2))
        } else {
          Some((Assignment, 1))
        }
      }
      '~' => {
        if self.source.is_match("~=".chars()) {
          Some((NotEquals, 2))
        } else {
          Some((BitwiseNot, 1))
        }
      }
      '<' => {
        if self.source.is_match("<<".chars()) {
          Some((LeftShift, 2))
        } else if self.source.is_match("<=".chars()) {
          Some((LessThanOrEqual, 2))
        } else {
          Some((LessThan, 1))
        }
      }
      '>' => {
        if self.source.is_match(">>".chars()) {
          Some((RightShift, 2))
        } else if self.source.is_match(">=".chars()) {
          Some((GreaterThanOrEqual, 2))
        } else {
          Some((GreaterThan, 1))
        }
      }
      '|' => {
        if self.source.is_match("|>".chars()) {
          Some((Pipe, 2))
        } else {
          Some((BitwiseOr, 1))
        }
      }
      '.' => {
        if self.source.is_match("..".chars()) {
          Some((Concat, 2))
        } else {
          Some((Index, 1))
        }
      }
      '&' => Some((BitwiseAnd, 1)),
      '^' => Some((BitwiseXor, 1)),
      '#' => Some((Length, 1)),
      '?' => {
        if self.source.is_match("??".chars()) {
          Some((OptionalCoalesce, 2))
        } else {
          None
        }
      }
      _ => None,
    }?;

    self.advance_by(advance_by);

    Some(self.new_token(TokenValue::Operator(operator)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Operator;
  use assertor::*;
  use moonjuice_common::Position;
  use rstest::rstest;

  #[rstest]
  #[case("+", Add)]
  #[case("-", Subtract)]
  #[case("*", Multiply)]
  #[case("/", Divide)]
  #[case("%", Modulo)]
  #[case("..", Concat)]
  #[case("#", Length)]
  #[case("??", OptionalCoalesce)]
  #[case("==", Equals)]
  #[case("~=", NotEquals)]
  #[case("<", LessThan)]
  #[case(">", GreaterThan)]
  #[case("<=", LessThanOrEqual)]
  #[case(">=", GreaterThanOrEqual)]
  #[case("|>", Pipe)]
  #[case(".", Index)]
  #[case("=", Assignment)]
  #[case("~", BitwiseNot)]
  #[case("&", BitwiseAnd)]
  #[case("|", BitwiseOr)]
  #[case("^", BitwiseXor)]
  #[case("<<", LeftShift)]
  #[case(">>", RightShift)]
  fn should_parse_operator(#[case] lexeme: &str, #[case] operator: Operator) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: TokenValue::Operator(operator),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }
}
