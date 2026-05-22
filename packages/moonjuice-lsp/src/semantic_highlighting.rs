use moonjuice_common::{Operator, Position};
use moonjuice_lexer::{Token, TokenValue};
use tower_lsp_server::ls_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend};

enum TokenType {
  Parameter = 0,
  Variable = 1,
  Property = 2,
  Function = 3,
  Keyword = 4,
  Comment = 5,
  String = 6,
  Number = 7,
  Operator = 8,
}

fn get_token_type(token: &Token) -> Option<TokenType> {
  match &token.value {
    TokenValue::Nil => Some(TokenType::Keyword),
    TokenValue::Bool(_) => Some(TokenType::Keyword),
    TokenValue::Int(_) => Some(TokenType::Number),
    TokenValue::Double(_) => Some(TokenType::Number),
    TokenValue::String(_, _) => Some(TokenType::String),
    TokenValue::Symbol(_) => Some(TokenType::Variable),
    TokenValue::Keyword(_) => Some(TokenType::Keyword),
    TokenValue::Operator(op) => match op {
      Operator::Add => Some(TokenType::Operator),
      Operator::Subtract => Some(TokenType::Operator),
      Operator::Multiply => Some(TokenType::Operator),
      Operator::Divide => Some(TokenType::Operator),
      Operator::Modulo => Some(TokenType::Operator),
      Operator::Concat => Some(TokenType::Operator),
      Operator::Length => Some(TokenType::Operator),
      Operator::Not => Some(TokenType::Keyword),
      Operator::And => Some(TokenType::Keyword),
      Operator::Or => Some(TokenType::Keyword),
      Operator::OptionalCoalesce => Some(TokenType::Operator),
      Operator::Equals => Some(TokenType::Operator),
      Operator::NotEquals => Some(TokenType::Operator),
      Operator::LessThan => Some(TokenType::Operator),
      Operator::GreaterThan => Some(TokenType::Operator),
      Operator::LessThanOrEqual => Some(TokenType::Operator),
      Operator::GreaterThanOrEqual => Some(TokenType::Operator),
      Operator::Pipe => Some(TokenType::Operator),
      Operator::Index => Some(TokenType::Operator),
      Operator::OptionalIndex => Some(TokenType::Operator),
      Operator::Assignment => Some(TokenType::Operator),
      Operator::BitwiseNot => Some(TokenType::Operator),
      Operator::BitwiseAnd => Some(TokenType::Operator),
      Operator::BitwiseOr => Some(TokenType::Operator),
      Operator::BitwiseXor => Some(TokenType::Operator),
      Operator::LeftShift => Some(TokenType::Operator),
      Operator::RightShift => Some(TokenType::Operator),
    },
    TokenValue::SpecialCharacter(_) => None,
    TokenValue::Comment(_) => Some(TokenType::Comment),
    TokenValue::UnexpectedCharacter(_) => None,
    TokenValue::MalformedNumber(_) => Some(TokenType::Number),
    TokenValue::MalformedString(_, _) => Some(TokenType::String),
  }
}

pub fn get_legend() -> SemanticTokensLegend {
  SemanticTokensLegend {
    token_types: vec![
      SemanticTokenType::PARAMETER,
      SemanticTokenType::VARIABLE,
      SemanticTokenType::PROPERTY,
      SemanticTokenType::FUNCTION,
      SemanticTokenType::KEYWORD,
      SemanticTokenType::COMMENT,
      SemanticTokenType::STRING,
      SemanticTokenType::NUMBER,
      SemanticTokenType::OPERATOR,
    ],
    token_modifiers: vec![SemanticTokenModifier::READONLY],
  }
}

pub fn convert_to_lsp_tokens<'a>(tokens: impl Iterator<Item = &'a Token>) -> Vec<SemanticToken> {
  let mut previous_start = Position { line: 1, column: 1 };

  tokens
    .filter_map(|token| {
      if let Some(token_type) = get_token_type(&token) {
        let token_type = token_type as u32;

        let mut delta_line = if token.start.line != previous_start.line {
          let delta_line = token.start.line - previous_start.line;

          previous_start = Position {
            line: token.start.line,
            column: 1,
          };

          delta_line as u32
        } else {
          0
        };
        let mut delta_start = (token.start.column - previous_start.column) as u32;

        if token.start.line == token.end.line {
          previous_start = token.start;

          Some(vec![SemanticToken {
            delta_line,
            delta_start,
            length: token.lexeme.len() as u32,
            token_type,
            token_modifiers_bitset: 0,
          }])
        } else {
          let mut tokens = Vec::<SemanticToken>::new();
          tokens.reserve(token.end.line - token.start.line + 1);

          for line in token.lexeme.lines() {
            tokens.push(SemanticToken {
              delta_line,
              delta_start,
              length: line.len() as u32,
              token_type,
              token_modifiers_bitset: 0,
            });

            delta_line = 1;
            delta_start = 0;
            previous_start = Position {
              line: previous_start.line + 1,
              column: 1,
            };
          }

          previous_start = Position {
            line: token.end.line,
            column: 1,
          };

          Some(tokens)
        }
      } else {
        None
      }
    })
    .flatten()
    .collect()
}
