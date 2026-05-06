use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenValue::Comment;

impl Lexer {
  pub(in crate::lexer) fn tokenise_comment(&mut self) -> Result<Option<Token>, Error> {
    if !self.source.is_match("--".chars()) {
      return Ok(None);
    }

    self.advance_by(2);

    let (comment_start, comment_end) = if self.source.is_match("[[".chars()) {
      self.advance_by(2);

      while !self.source.is_match("--]]".chars()) && self.source.has_next() {
        self.advance();
      }

      if self.source.is_match("--]]".chars()) {
        self.advance_by(4);

        (self.token_start_index + 4, self.source.get_index() - 4)
      } else {
        (self.token_start_index + 4, self.source.get_index())
      }
    } else {
      while !self.source.is_match("\n".chars()) && self.source.has_next() {
        self.advance()
      }

      (self.token_start_index + 2, self.source.get_index())
    };

    Ok(Some(
      self.new_token(Comment(self.read_string(comment_start..comment_end))),
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use assertor::*;
  use indoc::indoc;
  use moonjuice_common::Position;

  #[test]
  fn should_parse_single_line_comment() {
    let result = Lexer::tokenise("-- this is a comment".chars().collect());

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment(" this is a comment".to_string()),
      lexeme: "-- this is a comment".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 1, column: 21 },
    }]);
  }

  #[test]
  fn should_parse_multiline_comment() {
    let result = Lexer::tokenise(
      indoc! {"
      --[[
        This is a multi-line
        comment.
      --]]
    "}
      .chars()
      .collect(),
    );

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment("\n  This is a multi-line\n  comment.\n".to_string()),
      lexeme: "--[[\n  This is a multi-line\n  comment.\n--]]".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 5 },
    }]);
  }

  #[test]
  fn should_parse_multiline_comment_when_overruning() {
    let result = Lexer::tokenise(
      indoc! {"
      --[[
        This is a multi-line
        comment.
    "}
      .chars()
      .collect(),
    );

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment("\n  This is a multi-line\n  comment.\n".to_string()),
      lexeme: "--[[\n  This is a multi-line\n  comment.\n".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 1 },
    }]);
  }
}
