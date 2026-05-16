use crate::semantic_highlighting::convert_to_lsp_tokens;
use moonjuice_common::Position;
use moonjuice_lexer::{Lexer, Token};
use tower_lsp_server::ls_types::{
  Range, SemanticTokens, SemanticTokensRangeResult, SemanticTokensResult, TextDocumentContentChangeEvent,
};

pub struct Document {
  tokens: Vec<Token>,
}

impl Document {
  pub fn new(content: String) -> Self {
    Document {
      tokens: Lexer::tokenise(content.chars().collect()),
    }
  }

  pub fn apply_change(&mut self, changes: Vec<TextDocumentContentChangeEvent>) {
    if changes.len() != 1 {
      return;
    }

    let change = &changes[0];

    if change.range.is_some() {
      return;
    }

    self.tokens = Lexer::tokenise(change.text.chars().collect());
  }

  pub fn get_tokens_full(&self) -> SemanticTokensResult {
    SemanticTokensResult::Tokens(SemanticTokens {
      result_id: None,
      data: convert_to_lsp_tokens(self.tokens.iter()),
    })
  }

  pub fn get_tokens_range(&self, range: Range) -> SemanticTokensRangeResult {
    let start = Position {
      line: range.start.line as usize + 1,
      column: range.start.character as usize + 1,
    };
    let end = Position {
      line: range.end.line as usize + 1,
      column: range.end.character as usize + 1,
    };

    let tokens_in_range = self.tokens.iter().filter(|token| {
      token.start.line >= start.line
        && token.end.line <= end.line
        && (token.start.line > start.line || token.start.column >= start.column)
        && (token.start.line < end.line || token.start.column <= end.column)
    });

    SemanticTokensRangeResult::Tokens(SemanticTokens {
      result_id: None,
      data: convert_to_lsp_tokens(tokens_in_range),
    })
  }
}
