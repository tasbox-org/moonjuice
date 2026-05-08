use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::{Expression, ExpressionNode};
use moonjuice_lexer::StringTokenType::Whole;
use moonjuice_lexer::TokenValue;

impl Parser {
  pub(super) fn parse_string(&mut self) -> ExpressionNode {
    let start = self.get_start();

    match self.tokens.consume().map(|token| token.value.clone()) {
      Some(TokenValue::String(Whole, value)) => ExpressionNode {
        value: Expression::String {
          segments: vec![value],
          arguments: vec![],
        }
        .into(),
        start,
        end: self.get_end(),
      },
      Some(TokenValue::MalformedString(Whole, message)) => ExpressionNode {
        value: SyntaxError(message).into(),
        start,
        end: self.get_end(),
      },
      Some(TokenValue::String(_, _) | TokenValue::MalformedString(_, _)) => ExpressionNode {
        value: SyntaxError("Format strings are not yet implemented into the parser".to_string()).into(),
        start,
        end: self.get_end(),
      },
      _ => ExpressionNode {
        value: SyntaxError("Expected string".to_string()).into(),
        start,
        end: self.get_end(),
      },
    }
  }
}
