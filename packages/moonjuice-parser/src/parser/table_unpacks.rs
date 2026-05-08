use crate::Parser;
use crate::nodes::expression::{Expression, ExpressionNode};
use crate::nodes::lvalue::LValue::{SyntaxError, TableUnpack};
use crate::nodes::lvalue::{LValue, LValueNode, TableUnpackElement};
use moonjuice_common::Operator::{Assignment, Index};
use moonjuice_common::SpecialCharacter::{CloseCurlyBracket, CloseSquareBracket, OpenCurlyBracket, OpenSquareBracket};
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::{Operator, SpecialCharacter, Symbol};
use std::cell::Cell;

impl Parser {
  pub(super) fn parse_table_unpack(&mut self) -> LValueNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != SpecialCharacter(OpenCurlyBracket))
    {
      return LValueNode {
        value: SyntaxError("Expected '{' to open table unpack".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let next_numeric_index = Cell::new(1i64);

    let elements = self.consume_trailing_comma_separated(SpecialCharacter(CloseCurlyBracket), |p| {
      p.parse_table_unpack_element(&next_numeric_index)
    });

    if self.consume_if(SpecialCharacter(CloseCurlyBracket)).is_none() {
      return LValueNode {
        value: SyntaxError("Expected '}' to close table unpack".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    LValueNode {
      value: TableUnpack { elements }.into(),
      start,
      end: self.get_end(),
    }
  }

  fn parse_table_unpack_element(&mut self, next_numeric_index: &Cell<i64>) -> TableUnpackElement {
    let start = self.get_start();

    match self.tokens.peek_next().map(|token| token.value.clone()) {
      Some(Operator(Index)) => {
        self.tokens.consume();

        if let Some(Token {
          value: Symbol(symbol), ..
        }) = self
          .tokens
          .consume_if(|token| matches!(token.value, Symbol(_)))
          .cloned()
        {
          let key = ExpressionNode {
            value: Expression::String {
              segments: vec![symbol.clone()],
              arguments: vec![],
            }
            .into(),
            start: start.clone(),
            end: self.get_end(),
          };

          let variable = if self.consume_if(Operator(Assignment)).is_some() {
            self.parse_lvalue()
          } else {
            LValueNode {
              value: LValue::Symbol(symbol.clone()).into(),
              start,
              end: self.get_end(),
            }
          };

          TableUnpackElement::Valid { key, variable }
        } else {
          TableUnpackElement::SyntaxError("Expected key name after '.' in table unpack".to_string())
        }
      }
      Some(SpecialCharacter(OpenSquareBracket)) => {
        self.tokens.consume();

        let key = self.parse_expression();

        if self.consume_if(SpecialCharacter(CloseSquareBracket)).is_none() {
          TableUnpackElement::SyntaxError("Expected ']' to close table unpack index expression".to_string())
        } else if self.consume_if(Operator(Assignment)).is_none() {
          TableUnpackElement::SyntaxError("Expected '=' after table unpack index expression".to_string())
        } else {
          let variable = self.parse_lvalue();

          TableUnpackElement::Valid { key, variable }
        }
      }
      _ => {
        let variable = self.parse_lvalue();
        let key = ExpressionNode {
          value: Expression::Int(next_numeric_index.get()).into(),
          start,
          end: self.get_end(),
        };

        next_numeric_index.set(next_numeric_index.get() + 1);

        TableUnpackElement::Valid { key, variable }
      }
    }
  }
}
