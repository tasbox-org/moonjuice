use crate::Parser;
use crate::nodes::statement::Statement::SyntaxError;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::Break;
use moonjuice_lexer::TokenValue;

impl Parser {
  pub(super) fn parse_break(&mut self) -> StatementNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != TokenValue::Keyword(Break))
    {
      StatementNode {
        value: SyntaxError("Expected 'break' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      }
    } else {
      StatementNode {
        value: Statement::Break.into(),
        start,
        end: self.get_end(),
      }
    }
  }
}
