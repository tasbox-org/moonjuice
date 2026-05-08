use crate::Parser;
use crate::nodes::expression::Expression::{SyntaxError, TableDefinition};
use crate::nodes::expression::{Expression, ExpressionNode, TableDefinitionElement};
use moonjuice_common::Operator::{Assignment, Index};
use moonjuice_common::SpecialCharacter::{CloseCurlyBracket, CloseSquareBracket, OpenCurlyBracket, OpenSquareBracket};
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::{Operator, SpecialCharacter, Symbol};
use std::cell::Cell;

impl Parser {
  pub(super) fn parse_table_definition(&mut self) -> ExpressionNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != SpecialCharacter(OpenCurlyBracket))
    {
      return ExpressionNode {
        value: SyntaxError("Expected '{' to open table definition".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let next_numeric_index = Cell::new(1i64);

    let elements = self.consume_trailing_comma_separated(SpecialCharacter(CloseCurlyBracket), |p| {
      p.parse_table_definition_element(&next_numeric_index)
    });

    if self.consume_if(SpecialCharacter(CloseCurlyBracket)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected '}' to close table definition".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    ExpressionNode {
      value: TableDefinition { elements }.into(),
      start,
      end: self.get_end(),
    }
  }

  fn parse_table_definition_element(&mut self, next_numeric_index: &Cell<i64>) -> TableDefinitionElement {
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

          let value = if self.consume_if(Operator(Assignment)).is_some() {
            self.parse_expression()
          } else {
            ExpressionNode {
              value: Expression::Symbol(symbol.clone()).into(),
              start,
              end: self.get_end(),
            }
          };

          TableDefinitionElement::Valid { key, value }
        } else {
          TableDefinitionElement::SyntaxError("Expected key name after '.' in table definition".to_string())
        }
      }
      Some(SpecialCharacter(OpenSquareBracket)) => {
        self.tokens.consume();

        let key = self.parse_expression();

        if self.consume_if(SpecialCharacter(CloseSquareBracket)).is_none() {
          TableDefinitionElement::SyntaxError("Expected ']' to close table definition index expression".to_string())
        } else if self.consume_if(Operator(Assignment)).is_none() {
          TableDefinitionElement::SyntaxError("Expected '=' after table definition index expression".to_string())
        } else {
          let value = self.parse_expression();

          TableDefinitionElement::Valid { key, value }
        }
      }
      _ => {
        let value = self.parse_expression();
        let key = ExpressionNode {
          value: Expression::Int(next_numeric_index.get()).into(),
          start,
          end: self.get_end(),
        };

        next_numeric_index.set(next_numeric_index.get() + 1);

        TableDefinitionElement::Valid { key, value }
      }
    }
  }
}
