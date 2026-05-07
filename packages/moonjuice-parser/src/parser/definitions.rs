use crate::Parser;
use crate::nodes::expression::{Expression, ExpressionNode};
use crate::nodes::lvalue::{LValue, LValueNode};
use crate::nodes::statement::Statement::Definition;
use crate::nodes::statement::{Statement, StatementNode};
use moonjuice_common::Keyword::{Constant, Export, Mutable};
use moonjuice_common::Operator::Assignment;
use moonjuice_common::SpecialCharacter::Comma;
use moonjuice_lexer::TokenValue::{Keyword, Operator, SpecialCharacter};

impl Parser {
  pub(in crate::parser) fn parse_definition(&mut self) -> StatementNode {
    let start = self.get_start();

    if let Some(keyword) = self.tokens.consume()
      && matches!(keyword.value, Keyword(Constant | Mutable | Export))
    {
      let mut lhs: Vec<LValueNode> = vec![];
      loop {
        lhs.push(self.parse_lvalue());

        if self.consume_if(|value| value == SpecialCharacter(Comma)).is_none() {
          break;
        }
      }

      let lhs_end = self.get_end();

      if self.consume_if(|value| value == Operator(Assignment)).is_none() {
        return StatementNode {
          value: Statement::SyntaxError("Expected '=' following definition".to_string()).into(),
          start,
          end: self.get_end(),
        };
      }

      let mut rhs: Vec<ExpressionNode> = vec![];
      loop {
        rhs.push(self.parse_expression());

        if self.consume_if(|value| value == SpecialCharacter(Comma)).is_none() {
          break;
        }
      }

      let rhs_end = self.get_end();

      while lhs.len() < rhs.len() {
        lhs.push(LValueNode {
          value: LValue::SyntaxError("Expected variable or table unpack to match number of assignments".to_string())
            .into(),
          start: lhs_end.clone(),
          end: lhs_end.clone(),
        });
      }

      while rhs.len() < lhs.len() {
        rhs.push(ExpressionNode {
          value: Expression::SyntaxError(
            "Expected assignment to match number of variables or table unpacks".to_string(),
          )
          .into(),
          start: rhs_end.clone(),
          end: rhs_end.clone(),
        });
      }

      StatementNode {
        value: Definition {
          is_constant: false,
          is_export: false,
          definitions: lhs.into_iter().zip(rhs).collect(),
        }
        .into(),
        start,
        end: rhs_end,
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
