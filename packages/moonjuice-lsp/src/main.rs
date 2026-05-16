mod semantic_highlighting;

use crate::semantic_highlighting::SemanticHighlightingProvider;
use tower_lsp_server::ls_types::{
  InitializeParams, InitializeResult, InitializedParams, MessageType, SemanticTokensDeltaParams,
  SemanticTokensFullDeltaResult, SemanticTokensFullOptions, SemanticTokensOptions, SemanticTokensParams,
  SemanticTokensRangeParams, SemanticTokensRangeResult, SemanticTokensResult, SemanticTokensServerCapabilities,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

struct Backend {
  client: Client,
  semantic_highlighting_provider: SemanticHighlightingProvider,
}

impl LanguageServer for Backend {
  async fn initialize(&self, _params: InitializeParams) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
    let mut result = InitializeResult::default();
    result.capabilities.semantic_tokens_provider = Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
      SemanticTokensOptions {
        work_done_progress_options: Default::default(),
        legend: self.semantic_highlighting_provider.get_legend(),
        range: None,
        full: Some(SemanticTokensFullOptions::Bool(true)),
      },
    ));

    Ok(result)
  }

  async fn initialized(&self, _params: InitializedParams) {
    self.client.log_message(MessageType::INFO, "Server initialised").await;
  }

  async fn semantic_tokens_full(
    &self,
    params: SemanticTokensParams,
  ) -> tower_lsp_server::jsonrpc::Result<Option<SemanticTokensResult>> {
    self.semantic_highlighting_provider.highlight_full(params)
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
    semantic_highlighting_provider: SemanticHighlightingProvider::new(),
  });

  Server::new(stdin, stdout, socket).serve(service).await;
}
