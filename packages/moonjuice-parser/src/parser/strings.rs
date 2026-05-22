use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::StringSegment::{Malformed, Valid};
use crate::nodes::expression::{Expression, ExpressionNode, StringSegmentNode};
use moonjuice_lexer::StringTokenType::{End, Middle, Start, Whole};
use moonjuice_lexer::TokenValue;

impl Parser {
  pub(super) fn parse_string(&mut self) -> ExpressionNode {
    let start = self.get_start();

    match self.tokens.peek_next().map(|token| token.value.clone()) {
      Some(TokenValue::String(Whole, value)) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::String {
            segments: vec![StringSegmentNode {
              value: Valid(value).into(),
              start,
              end: self.get_end(),
            }],
            arguments: vec![],
          }
          .into(),
          start,
          end: self.get_end(),
        }
      }
      Some(TokenValue::MalformedString(Whole, message)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError(message).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(TokenValue::String(Start, _) | TokenValue::MalformedString(Start, _)) => self.parse_format_string(),
      Some(TokenValue::String(_, _) | TokenValue::MalformedString(_, _)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError(
            "Found middle/end format string segment as start of string expression (this should never happen!)"
              .to_string(),
          )
          .into(),
          start,
          end: self.get_end(),
        }
      }
      _ => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError("Expected string".to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
    }
  }

  fn parse_format_string(&mut self) -> ExpressionNode {
    let start = self.get_start();

    let mut segments: Vec<StringSegmentNode> = vec![];
    let mut arguments: Vec<ExpressionNode> = vec![];

    loop {
      match self.tokens.peek_next().map(|token| token.value.clone()) {
        Some(TokenValue::String(Start | Middle, value)) => {
          self.tokens.consume();
          segments.push(StringSegmentNode {
            value: Valid(value).into(),
            start,
            end: self.get_end(),
          })
        }
        Some(TokenValue::String(End, value)) => {
          self.tokens.consume();
          segments.push(StringSegmentNode {
            value: Valid(value).into(),
            start,
            end: self.get_end(),
          });

          break;
        }
        Some(TokenValue::MalformedString(Start | Middle, message)) => {
          self.tokens.consume();
          segments.push(StringSegmentNode {
            value: Malformed(message).into(),
            start,
            end: self.get_end(),
          })
        }
        Some(TokenValue::MalformedString(End, message)) => {
          self.tokens.consume();
          segments.push(StringSegmentNode {
            value: Malformed(message).into(),
            start,
            end: self.get_end(),
          });

          break;
        }
        _ => {
          return ExpressionNode {
            value: SyntaxError("Format string is unterminated".to_string()).into(),
            start,
            end: self.get_end(),
          };
        }
      }

      match self.tokens.peek_next().map(|token| token.value.clone()) {
        Some(TokenValue::String(Middle | End, _) | TokenValue::MalformedString(Middle | End, _)) => {
          // Note that we don't consume here. We'll consume it on the next iteration when parsing the next segment
          arguments.push(ExpressionNode {
            value: SyntaxError("Format string holes must not be empty".to_string()).into(),
            start: self.get_end(),
            end: self.tokens.peek_next().unwrap().start,
          })
        }
        None => {
          // We fall through if this ran off the end of the program, as it'll be caught on the next iteration above
        }
        _ => arguments.push(self.parse_expression()),
      }
    }

    ExpressionNode {
      value: Expression::String { segments, arguments }.into(),
      start,
      end: self.get_end(),
    }
  }
}
