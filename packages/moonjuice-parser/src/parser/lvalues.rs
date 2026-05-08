use crate::Parser;
use crate::nodes::lvalue::LValue::SyntaxError;
use crate::nodes::lvalue::{LValue, LValueNode};
use moonjuice_common::SpecialCharacter::OpenCurlyBracket;
use moonjuice_lexer::TokenValue::{SpecialCharacter, Symbol};

impl Parser {
  pub(super) fn parse_lvalue(&mut self) -> LValueNode {
    match self.tokens.peek_next().map(|token| token.value.clone()) {
      Some(Symbol(symbol)) => {
        let start = self.get_start();
        self.tokens.consume();

        LValueNode {
          value: LValue::Symbol(symbol).into(),
          start,
          end: self.get_end(),
        }
      }
      Some(SpecialCharacter(OpenCurlyBracket)) => self.parse_table_unpack(),
      _ => {
        let start = self.get_start();
        self.tokens.consume();

        LValueNode {
          value: SyntaxError("Expected variable name or table unpack".to_string()).into(),
          start,
          end: self.get_end(),
        }
      }
    }
  }
}
