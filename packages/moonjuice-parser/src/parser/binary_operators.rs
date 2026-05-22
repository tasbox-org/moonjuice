use crate::Parser;
use crate::nodes::expression::Expression::{BinaryOperator, Symbol, SyntaxError};
use crate::nodes::expression::{Expression, ExpressionNode, StringSegment, StringSegmentNode};
use crate::operators::{OperatorMetadata, get_operator_metadata};
use moonjuice_common::Operator::{Assignment, Index, OptionalIndex};
use moonjuice_lexer::TokenValue::Operator;

fn is_right_hand_side_chained_operator(metadata: &OperatorMetadata, previous_precedence: u16) -> bool {
  if metadata.is_unary {
    return false;
  }

  if metadata.is_right_associative {
    metadata.precedence >= previous_precedence
  } else {
    metadata.precedence > previous_precedence
  }
}

impl Parser {
  pub(super) fn parse_binary_operator(&mut self, lhs: ExpressionNode) -> ExpressionNode {
    let start = self.get_start();
    let token = self.tokens.consume().cloned();
    let end = self.get_end();

    if let Some(token) = token
      && let Operator(op) = token.value.clone()
      && let Some(metadata) = get_operator_metadata(token)
    {
      let mut rhs = self.parse_operand();

      while let Some(lookahead) = self.tokens.peek_next().cloned()
        && let Some(lookahead_metadata) = get_operator_metadata(lookahead)
        && is_right_hand_side_chained_operator(&lookahead_metadata, metadata.precedence)
      {
        rhs = self.parse_sub_expression(
          rhs,
          if lookahead_metadata.precedence > metadata.precedence {
            metadata.precedence + 1
          } else {
            metadata.precedence
          },
        );
      }

      match op {
        Assignment => {
          let is_valid_for_assignment = matches!(*lhs.value, Symbol(_) | BinaryOperator { op: Index, .. });
          if is_valid_for_assignment {
            ExpressionNode {
              value: BinaryOperator { op, lhs, rhs }.into(),
              start,
              end,
            }
          } else {
            ExpressionNode {
              value: SyntaxError(
                "Expected symbol or index expression (without optional chaining) on left side of assignment operator"
                  .to_string(),
              )
              .into(),
              start: lhs.start,
              end: lhs.end,
            }
          }
        }
        Index | OptionalIndex => {
          if let Symbol(symbol) = *rhs.value {
            ExpressionNode {
              value: BinaryOperator {
                op,
                lhs,
                rhs: ExpressionNode {
                  value: Expression::String {
                    segments: vec![StringSegmentNode {
                      value: StringSegment::Valid(symbol).into(),
                      start: rhs.start,
                      end: rhs.end,
                    }],
                    arguments: vec![],
                  }
                  .into(),
                  start: rhs.start,
                  end: rhs.end,
                },
              }
              .into(),
              start,
              end,
            }
          } else {
            ExpressionNode {
              value: SyntaxError("Expected symbol on right-hand side of index operator".to_string()).into(),
              start: rhs.start,
              end: rhs.end,
            }
          }
        }
        _ => ExpressionNode {
          value: BinaryOperator { op, lhs, rhs }.into(),
          start,
          end,
        },
      }
    } else {
      ExpressionNode {
        value: SyntaxError("Expected binary operator".to_string()).into(),
        start,
        end,
      }
    }
  }
}
