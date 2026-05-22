use crate::diagnostics::DiagnosticsBuilder;
use crate::semantic_highlighting::convert_to_lsp_tokens;
use moonjuice_common::Position;
use moonjuice_lexer::{Lexer, Token};
use moonjuice_parser::Parser;
use tower_lsp_server::ls_types::{
  Diagnostic, Range, SemanticTokens, SemanticTokensRangeResult, SemanticTokensResult, TextDocumentContentChangeEvent,
};

pub struct Document {
  tokens: Vec<Token>,
  pub diagnostics: Vec<Diagnostic>,
}

impl Document {
  pub fn new(content: String) -> Self {
    let tokens = Lexer::tokenise(content.chars().collect());
    let ast = Parser::parse(tokens.clone());

    Document {
      tokens,
      diagnostics: DiagnosticsBuilder::new().build(&ast),
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

    let ast = Parser::parse(self.tokens.clone());
    self.diagnostics = DiagnosticsBuilder::new().build(&ast);
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
      token.end.line >= start.line
        && token.start.line <= end.line
        && (token.end.line > start.line || token.end.column >= start.column)
        && (token.start.line < end.line || token.start.column <= end.column)
    });

    SemanticTokensRangeResult::Tokens(SemanticTokens {
      result_id: None,
      data: convert_to_lsp_tokens(tokens_in_range),
    })
  }
}
