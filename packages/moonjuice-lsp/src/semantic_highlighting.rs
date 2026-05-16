use moonjuice_common::Position;
use moonjuice_lexer::{Lexer, Token, TokenValue};
use std::fs;
use tower_lsp_server::ls_types::{
  SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensLegend, SemanticTokensParams,
  SemanticTokensResult,
};

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

pub struct SemanticHighlightingProvider {}

impl SemanticHighlightingProvider {
  pub fn new() -> Self {
    SemanticHighlightingProvider {}
  }

  pub fn get_legend(&self) -> SemanticTokensLegend {
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

  pub fn highlight_full(
    &self,
    params: SemanticTokensParams,
  ) -> tower_lsp_server::jsonrpc::Result<Option<SemanticTokensResult>> {
    if let Some(file_path) = params.text_document.uri.to_file_path() {
      let contents = fs::read_to_string(file_path)
        .map_err(|err| tower_lsp_server::jsonrpc::Error::invalid_params(err.to_string()))?;

      let mut previous_start = Position { line: 1, column: 1 };
      let tokens = Lexer::tokenise(contents.chars().collect())
        .into_iter()
        .filter_map(|token| {
          if token.start.line != previous_start.line {
            previous_start = Position {
              line: token.start.line,
              column: 1,
            };
          }

          if let Some(token_type) = Self::get_token_type(&token) {
            let semantic_token = SemanticToken {
              delta_line: (token.start.line - 1) as u32,
              delta_start: (token.start.column - previous_start.column) as u32,
              length: token.lexeme.len() as u32,
              token_type: token_type as u32,
              token_modifiers_bitset: 0,
            };

            previous_start = token.start;
            Some(semantic_token)
          } else {
            None
          }
        })
        .collect();

      Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: tokens,
      })))
    } else {
      Err(tower_lsp_server::jsonrpc::Error::invalid_params(
        "Non-file text document URIs are not supported",
      ))
    }
  }

  fn get_token_type(token: &Token) -> Option<TokenType> {
    match token.value {
      TokenValue::Nil => Some(TokenType::Keyword),
      TokenValue::Bool(_) => Some(TokenType::Keyword),
      TokenValue::Int(_) => Some(TokenType::Number),
      TokenValue::Double(_) => Some(TokenType::Number),
      TokenValue::String(_, _) => Some(TokenType::String),
      TokenValue::Symbol(_) => Some(TokenType::Variable),
      TokenValue::Keyword(_) => Some(TokenType::Keyword),
      TokenValue::Operator(_) => Some(TokenType::Operator),
      TokenValue::SpecialCharacter(_) => None,
      TokenValue::Comment(_) => Some(TokenType::Comment),
      TokenValue::UnexpectedCharacter(_) => None,
      TokenValue::MalformedNumber(_) => Some(TokenType::Number),
      TokenValue::MalformedString(_, _) => Some(TokenType::String),
    }
  }
}
