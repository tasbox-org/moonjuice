use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::{Expression, ExpressionNode};
use moonjuice_common::Keyword::{Do, End, For, Function, If};
use moonjuice_common::SpecialCharacter::{OpenBracket, OpenCurlyBracket};
use moonjuice_lexer::TokenValue::{
  Bool, Comment, Double, Int, Keyword, MalformedNumber, MalformedString, Nil, Operator, SpecialCharacter, String,
  Symbol, UnexpectedCharacter,
};

impl Parser {
  pub(super) fn parse_operand(&mut self) -> ExpressionNode {
    let start = self.get_start();

    match self.tokens.peek_next().map(|token| token.value.clone()) {
      Some(Operator(_)) => self.parse_unary_operator(),
      Some(SpecialCharacter(OpenBracket)) => {
        self.tokens.consume();
        let inner = self.parse_expression();

        if self
          .consume_if(|value| value == SpecialCharacter(OpenCurlyBracket))
          .is_some()
        {
          inner
        } else {
          ExpressionNode {
            value: SyntaxError("Expected ')' to close inner expression".to_string()).into(),
            start,
            end: self.get_end(),
          }
        }
      }
      Some(SpecialCharacter(OpenCurlyBracket)) => self.parse_table_definition(),
      Some(SpecialCharacter(_)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError("Unexpected special character in expression".to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Keyword(Do)) => {
        self.tokens.consume();
        let block = self.parse_block(|p| p.tokens.peek_next().is_some_and(|token| token.value == Keyword(End)));

        if self.consume_if(|value| value == Keyword(End)).is_some() {
          block
        } else {
          ExpressionNode {
            value: SyntaxError("Expected 'end' to close 'do' block".to_string()).into(),
            start,
            end: self.get_end(),
          }
        }
      }
      Some(Keyword(Function)) => self.parse_function(),
      Some(Keyword(If)) => self.parse_if(),
      Some(Keyword(For)) => self.parse_for(),
      Some(Keyword(_)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError("Unexpected keyword in expression".to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Nil) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::Nil.into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Bool(bool)) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::Bool(bool).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Int(int)) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::Int(int).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Double(double)) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::Double(double).into(),
          start,
          end: self.get_end(),
        }
      }

      // Malformed strings don't immediately convert to syntax error nodes
      // as we still handle the structure of format strings with malformed segments
      Some(String(_, _) | MalformedString(_, _)) => self.parse_string(),

      Some(Symbol(symbol)) => {
        self.tokens.consume();

        ExpressionNode {
          value: Expression::Symbol(symbol).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(Comment(_)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError("Comments were not stripped from parser tokens (this should not happen!)".to_string())
            .into(),
          start,
          end: self.get_end(),
        }
      }
      Some(UnexpectedCharacter(char)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError(format!("Unexpected character '{}'", char).to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(MalformedNumber(message)) => {
        self.tokens.consume();

        ExpressionNode {
          value: SyntaxError(format!("Malformed number: {}", message).to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
      None => ExpressionNode {
        value: SyntaxError("Expression ran off the end of the program".to_string()).into(),
        start,
        end: self.get_end(),
      },
    }
  }
}
