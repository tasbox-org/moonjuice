use crate::Parser;
use crate::nodes::expression::Expression::SyntaxError;
use crate::nodes::expression::{Expression, ExpressionNode};
use moonjuice_common::Keyword::{End, Function};
use moonjuice_common::SpecialCharacter::{CloseBracket, OpenBracket};
use moonjuice_lexer::TokenValue::{Keyword, SpecialCharacter};

impl Parser {
  pub(super) fn parse_function_definition(&mut self) -> ExpressionNode {
    let start = self.get_start();

    if self
      .tokens
      .consume()
      .is_none_or(|token| token.value != Keyword(Function))
    {
      return ExpressionNode {
        value: SyntaxError("Expected 'function' keyword".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    if self.consume_if(SpecialCharacter(OpenBracket)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected '(' to open function parameter list".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let parameters = if self.is_next(SpecialCharacter(CloseBracket)) {
      vec![]
    } else {
      self.consume_comma_separated(|p| p.parse_lvalue())
    };

    if self.consume_if(SpecialCharacter(CloseBracket)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected ')' to close function parameter list".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    let body = self.parse_block(|p| p.tokens.has_next() && !p.is_next(Keyword(End)));

    if self.consume_if(Keyword(End)).is_none() {
      return ExpressionNode {
        value: SyntaxError("Expected 'end' keyword to close function body".to_string()).into(),
        start,
        end: self.get_end(),
      };
    }

    ExpressionNode {
      value: Expression::Function { parameters, body }.into(),
      start,
      end: self.get_end(),
    }
  }
}
