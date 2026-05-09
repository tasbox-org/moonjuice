use crate::{Error, LuauTranspiler};
use moonjuice_common::Operator::Index;
use moonjuice_common::Position;
use moonjuice_parser::nodes::expression::Expression::{BinaryOperator, Symbol};
use moonjuice_parser::nodes::expression::ExpressionNode;
use moonjuice_parser::nodes::lvalue::{LValue, LValueNode, TableUnpackElement};
use moonjuice_parser::nodes::statement::Statement::Definition;
use moonjuice_parser::nodes::statement::StatementNode;

impl LuauTranspiler {
  pub(super) fn emit_lvalue(&mut self, lvalue: LValueNode) -> Result<(), Error> {
    let start = lvalue.start;
    let end = lvalue.end;

    match *lvalue.value {
      LValue::Symbol(value) => self.emit_symbol(value.as_str()),
      LValue::TableUnpack { elements } => self.emit_table_unpack(elements, start, end)?,
      LValue::SyntaxError(message) => return Err(Error { message, start, end }),
    }

    Ok(())
  }

  fn emit_table_unpack(
    &mut self,
    elements: Vec<TableUnpackElement>,
    start: Position,
    end: Position,
  ) -> Result<(), Error> {
    let symbol = format!("tbl_{}", self.get_unique_id());
    self.source.push_str(symbol.as_str());

    let mut definition_lhs = vec![];
    let mut definition_rhs = vec![];

    for element in elements {
      match element {
        TableUnpackElement::Valid { key, variable } => {
          let index = ExpressionNode {
            value: BinaryOperator {
              op: Index,
              lhs: ExpressionNode {
                value: Symbol(symbol.clone()).into(),
                start,
                end,
              },
              rhs: key,
            }
            .into(),
            start,
            end,
          };

          definition_lhs.push(variable);
          definition_rhs.push(index);
        }
        TableUnpackElement::SyntaxError(message) => {
          return Err(Error { message, start, end });
        }
      }
    }

    let unpack = StatementNode {
      value: Definition {
        is_constant: true,
        is_export: false,
        lhs: definition_lhs,
        rhs: definition_rhs,
      }
      .into(),
      start,
      end,
    };

    self.get_scope_mut().table_unpacks.push(unpack);

    Ok(())
  }
}
