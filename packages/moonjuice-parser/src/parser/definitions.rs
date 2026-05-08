use crate::Parser;
use crate::nodes::statement::Statement::Definition;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::{Constant, Export, Mutable};
use moonjuice_common::Operator::Assignment;
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::{Keyword, Operator};

impl Parser {
  pub(super) fn parse_definition(&mut self) -> StatementNode {
    let start = self.get_start();

    if let Some(Token {
      value: Keyword(keyword),
      ..
    }) = self.tokens.consume().cloned()
      && matches!(keyword, Constant | Mutable | Export)
    {
      let lhs = self.consume_comma_separated(|p| p.parse_lvalue());

      if self.consume_if(Operator(Assignment)).is_none() {
        return StatementNode {
          value: Statement::SyntaxError("Expected '=' following definition".to_string()).into(),
          start,
          end: self.get_end(),
        };
      }

      let rhs = self.consume_comma_separated(|p| p.parse_expression());

      StatementNode {
        value: Definition {
          is_constant: matches!(keyword, Constant | Export),
          is_export: keyword == Export,
          lhs,
          rhs,
        }
        .into(),
        start,
        end: self.get_end(),
      }
    } else {
      StatementNode {
        value: Statement::SyntaxError("Expected 'def', 'mut' or 'export'".to_string()).into(),
        start,
        end: self.get_end(),
      }
    }
  }
}
