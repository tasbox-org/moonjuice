use crate::Parser;
use crate::nodes::statement::Statement::SyntaxError;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::Return;
use moonjuice_lexer::TokenValue;

impl Parser {
  pub(in crate::parser) fn parse_return(&mut self) -> StatementNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != TokenValue::Keyword(Return))
    {
      StatementNode {
        value: SyntaxError("Expected 'return' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      }
    } else {
      StatementNode {
        value: Statement::Return(self.parse_expression()).into(),
        start,
        end: self.get_end(),
      }
    }
  }
}
