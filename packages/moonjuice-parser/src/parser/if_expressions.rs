use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::{Expression, ExpressionNode, IfBranch};
use moonjuice_common::Keyword::{Else, ElseIf, End, If, Then};
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::Keyword;

impl Parser {
  pub(super) fn parse_if(&mut self) -> ExpressionNode {
    let start = self.get_start();

    if self.tokens.consume().is_none_or(|token| token.value != Keyword(If)) {
      return ExpressionNode {
        value: SyntaxError("Expected 'if' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let mut if_branches: Vec<IfBranch> = vec![];

    loop {
      let condition = self.parse_expression();

      if self.consume_if(|value| value == Keyword(Then)).is_none() {
        return ExpressionNode {
          value: SyntaxError("Expected 'then' after condition".to_string()).into(),
          start,
          end: self.get_end(),
        };
      }

      let body = self.parse_block(|p| {
        !matches!(
          p.tokens.peek_next(),
          Some(Token {
            value: Keyword(End | Else | ElseIf),
            ..
          }) | None
        )
      });

      if !matches!(
        self.tokens.peek_next(),
        Some(Token {
          value: Keyword(End | Else | ElseIf),
          ..
        })
      ) {
        return ExpressionNode {
          value: SyntaxError("Expected 'end', 'else', or 'elseif' after if expression body".to_string()).into(),
          start,
          end: self.get_end(),
        };
      }

      if_branches.push(IfBranch { condition, body });

      if self.consume_if(|value| value == Keyword(ElseIf)).is_none() {
        break;
      }
    }

    let else_branch = if self.consume_if(|value| value == Keyword(Else)).is_some() {
      Some(self.parse_block(|p| p.tokens.has_next() && !p.is_next(Keyword(End))))
    } else {
      None
    };

    if self.consume_if(|value| value == Keyword(End)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected 'end' to close if expression".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    ExpressionNode {
      value: Expression::If {
        if_branches,
        else_branch,
      }
      .into(),
      start,
      end: self.get_end(),
    }
  }
}
