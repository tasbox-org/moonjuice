use crate::Parser;
use crate::nodes::statement::Statement::{Return, SyntaxError};
use crate::nodes::statement::StatementNode;
use moonjuice_lexer::TokenValue;

impl Parser {
  pub(in crate::parser) fn parse_return(&mut self) -> StatementNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| matches!(token.value, TokenValue::Keyword(_)))
    {
      StatementNode {
        value: SyntaxError("Expected 'return' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      }
    } else {
      StatementNode {
        value: Return(self.parse_expression()).into(),
        start,
        end: self.get_end(),
      }
    }
  }
}
