mod document;
mod semantic_highlighting;

use crate::document::Document;
use crate::semantic_highlighting::get_legend;
use dashmap::DashMap;
use tower_lsp_server::ls_types::{
  DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DocumentFilter, InitializeParams,
  InitializeResult, InitializedParams, MessageType, SemanticTokensFullOptions, SemanticTokensOptions,
  SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult, SemanticTokensRegistrationOptions,
  SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities, TextDocumentRegistrationOptions,
  TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

struct Backend {
  client: Client,
  documents: DashMap<String, Document>,
}

impl LanguageServer for Backend {
  async fn initialize(&self, _params: InitializeParams) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
    let result = InitializeResult {
      capabilities: ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
          open_close: Some(true),
          change: Some(TextDocumentSyncKind::FULL),
          save: None,
          ..Default::default()
        })),

        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
          SemanticTokensRegistrationOptions {
            text_document_registration_options: TextDocumentRegistrationOptions {
              document_selector: Some(vec![DocumentFilter {
                language: Some("moonjuice".to_string()),
                scheme: Some("file".to_string()),
                pattern: None,
              }]),
            },
            semantic_tokens_options: SemanticTokensOptions {
              work_done_progress_options: Default::default(),
              legend: get_legend(),
              range: Some(true),
              full: Some(SemanticTokensFullOptions::Bool(true)),
            },
            static_registration_options: Default::default(),
          },
        )),

        ..Default::default()
      },
      ..Default::default()
    };

    Ok(result)
  }

  async fn initialized(&self, _params: InitializedParams) {
    self.client.log_message(MessageType::INFO, "Server initialised").await;
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    self.documents.insert(
      params.text_document.uri.to_string(),
      Document::new(params.text_document.text),
    );
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    if let Some(mut document) = self.documents.get_mut(params.text_document.uri.as_str()) {
      document.value_mut().apply_change(params.content_changes);
    }
  }

  async fn did_close(&self, params: DidCloseTextDocumentParams) {
    self.documents.remove(params.text_document.uri.as_str());
  }

  async fn semantic_tokens_full(
    &self,
    params: SemanticTokensParams,
  ) -> tower_lsp_server::jsonrpc::Result<Option<SemanticTokensResult>> {
    Ok(
      self
        .documents
        .get(params.text_document.uri.as_str())
        .map(|document| document.get_tokens_full()),
    )
  }

  async fn semantic_tokens_range(
    &self,
    params: SemanticTokensRangeParams,
  ) -> tower_lsp_server::jsonrpc::Result<Option<SemanticTokensRangeResult>> {
    Ok(
      self
        .documents
        .get(params.text_document.uri.as_str())
        .map(|document| document.get_tokens_range(params.range)),
    )
  }

  async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
    Ok(())
  }
}

#[tokio::main]
async fn main() {
  let stdin = tokio::io::stdin();
  let stdout = tokio::io::stdout();

  let (service, socket) = LspService::new(|client| Backend {
    client,
    documents: Default::default(),
  });

  Server::new(stdin, stdout, socket).serve(service).await;
}
