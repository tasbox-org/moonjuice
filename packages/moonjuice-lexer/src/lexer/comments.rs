use crate::Token;
use crate::TokenValue::Comment;
use crate::lexer::Lexer;

impl Lexer {
  pub(super) fn tokenise_comment(&mut self) -> Option<Token> {
    if !self.source.is_match("--".chars()) {
      return None;
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

    Some(self.new_token(Comment(self.read_string(comment_start..comment_end))))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Token;
  use crate::TokenValue::Comment;
  use assertor::*;
  use indoc::indoc;
  use moonjuice_common::Position;

  #[test]
  fn should_parse_single_line_comment() {
    let tokens = Lexer::tokenise("-- this is a comment".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment(" this is a comment".to_string()),
      lexeme: "-- this is a comment".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 1, column: 21 },
    }]);
  }

  #[test]
  fn should_parse_multiline_comment() {
    let tokens = Lexer::tokenise(
      indoc! {"
        --[[
          This is a multi-line
          comment.
        --]]
      "}
      .chars()
      .collect(),
    );

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment("\n  This is a multi-line\n  comment.\n".to_string()),
      lexeme: "--[[\n  This is a multi-line\n  comment.\n--]]".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 5 },
    }]);
  }

  #[test]
  fn should_parse_multiline_comment_when_overrunning() {
    let tokens = Lexer::tokenise(
      indoc! {"
        --[[
          This is a multi-line
          comment.
      "}
      .chars()
      .collect(),
    );

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Comment("\n  This is a multi-line\n  comment.\n".to_string()),
      lexeme: "--[[\n  This is a multi-line\n  comment.\n".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 4, column: 1 },
    }]);
  }
}
