use moonjuice_parser::nodes::Node;
use moonjuice_parser::nodes::expression::{
  Expression, ExpressionNode, StringSegment, StringSegmentNode, TableDefinitionElement, TableDefinitionElementNode,
};
use moonjuice_parser::nodes::lvalue::{LValue, LValueNode, TableUnpackElement, TableUnpackElementNode};
use moonjuice_parser::nodes::statement::{Statement, StatementNode};
use tower_lsp_server::ls_types::{Diagnostic, DiagnosticSeverity, Position, Range};

fn to_lsp_range<T>(node: &Node<T>) -> Range {
  let start = node.start;
  let end = node.end;

  Range {
    start: Position {
      line: (start.line - 1) as u32,
      character: (start.column - 1) as u32,
    },
    end: Position {
      line: (end.line - 1) as u32,
      character: (end.column - 1) as u32,
    },
  }
}

fn to_lsp_diagnostic<T>(node: &Node<T>, error: &String) -> Diagnostic {
  Diagnostic {
    range: to_lsp_range(node),
    severity: Some(DiagnosticSeverity::ERROR),
    code: None,
    code_description: None,
    source: Some("moonjuice".to_string()),
    message: error.clone(),
    related_information: None,
    tags: None,
    data: None,
  }
}

pub struct DiagnosticsBuilder {
  diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsBuilder {
  pub fn new() -> Self {
    DiagnosticsBuilder {
      diagnostics: Vec::new(),
    }
  }

  pub fn build(mut self, ast: &Vec<StatementNode>) -> Vec<Diagnostic> {
    for node in ast {
      self.visit_statement_node(node);
    }

    self.diagnostics
  }

  fn visit_statement_node(&mut self, statement: &StatementNode) {
    match statement.value.as_ref() {
      Statement::Definition { lhs, rhs, .. } => {
        for lvalue in lhs {
          self.visit_lvalue_node(lvalue);
        }

        for rvalue in rhs {
          self.visit_expression_node(rvalue);
        }
      }
      Statement::Return(expression) => self.visit_expression_node(expression),
      Statement::Break => {}
      Statement::Expression(expression) => self.visit_expression(statement, expression),
      Statement::SyntaxError(error) => self.diagnostics.push(to_lsp_diagnostic(statement, error)),
    }
  }

  fn visit_lvalue_node(&mut self, lvalue: &LValueNode) {
    match lvalue.value.as_ref() {
      LValue::Symbol(_) => {}
      LValue::TableUnpack { elements } => {
        for element in elements {
          self.visit_table_unpack_element_node(element);
        }
      }
      LValue::SyntaxError(error) => self.diagnostics.push(to_lsp_diagnostic(lvalue, error)),
    }
  }

  fn visit_table_unpack_element_node(&mut self, element: &TableUnpackElementNode) {
    match element.value.as_ref() {
      TableUnpackElement::Valid { key, variable } => {
        self.visit_expression_node(key);
        self.visit_lvalue_node(variable);
      }
      TableUnpackElement::SyntaxError(error) => self.diagnostics.push(to_lsp_diagnostic(element, error)),
    }
  }

  fn visit_expression_node(&mut self, expression: &ExpressionNode) {
    self.visit_expression(expression, expression.value.as_ref());
  }

  fn visit_expression<T>(&mut self, node: &Node<T>, expression: &Expression) {
    match expression {
      Expression::Nil => {}
      Expression::Bool(_) => {}
      Expression::Int(_) => {}
      Expression::Double(_) => {}
      Expression::String { segments, arguments } => {
        for segment in segments {
          self.visit_string_segment(segment);
        }

        for argument in arguments {
          self.visit_expression_node(argument);
        }
      }
      Expression::TableDefinition { elements } => {
        for element in elements {
          self.visit_table_definition_element(element);
        }
      }
      Expression::Symbol(_) => {}
      Expression::Block(body) => {
        for statement in body {
          self.visit_statement_node(statement);
        }
      }
      Expression::UnaryOperator { rhs, .. } => self.visit_expression_node(rhs),
      Expression::BinaryOperator { lhs, rhs, .. } => {
        self.visit_expression_node(lhs);
        self.visit_expression_node(rhs);
      }
      Expression::Function { parameters, body } => {
        for parameter in parameters {
          self.visit_lvalue_node(parameter);
        }

        for statement in body {
          self.visit_statement_node(statement);
        }
      }
      Expression::If {
        if_branches,
        else_branch,
      } => {
        for branch in if_branches {
          self.visit_expression_node(&branch.condition);

          for statement in &branch.body {
            self.visit_statement_node(&statement);
          }
        }

        if let Some(else_branch) = else_branch {
          for statement in else_branch {
            self.visit_statement_node(statement);
          }
        }
      }
      Expression::For { lhs, enumerable, body } => {
        for lvalue in lhs {
          self.visit_lvalue_node(lvalue);
        }

        self.visit_expression_node(enumerable);

        for statement in body {
          self.visit_statement_node(statement);
        }
      }
      Expression::Call { lhs, arguments, .. } => {
        self.visit_expression_node(lhs);

        for argument in arguments {
          self.visit_expression_node(argument);
        }
      }
      Expression::SyntaxError(error) => self.diagnostics.push(to_lsp_diagnostic(node, error)),
    }
  }

  fn visit_string_segment(&mut self, segment: &StringSegmentNode) {
    match segment.value.as_ref() {
      StringSegment::Valid(_) => {}
      StringSegment::Malformed(error) => self.diagnostics.push(to_lsp_diagnostic(segment, error)),
    }
  }

  fn visit_table_definition_element(&mut self, element: &TableDefinitionElementNode) {
    match element.value.as_ref() {
      TableDefinitionElement::Valid { key, value } => {
        self.visit_expression_node(key);
        self.visit_expression_node(value);
      }
      TableDefinitionElement::SyntaxError(error) => self.diagnostics.push(to_lsp_diagnostic(element, error)),
    }
  }
}
