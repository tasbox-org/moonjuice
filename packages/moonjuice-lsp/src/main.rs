use tower_lsp_server::ls_types::{
  InitializeParams, InitializeResult, InitializedParams, MessageType, SemanticTokensParams, SemanticTokensResult,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
  client: Client,
}

impl LanguageServer for Backend {
  async fn initialize(&self, params: InitializeParams) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
    Ok(InitializeResult::default())
  }

  async fn initialized(&self, params: InitializedParams) {
    self.client.log_message(MessageType::INFO, "Server initialised").await;
  }

  async fn semantic_tokens_full(
    &self,
    params: SemanticTokensParams,
  ) -> tower_lsp_server::jsonrpc::Result<Option<SemanticTokensResult>> {
    self
      .client
      .log_message(MessageType::INFO, format!("Semantic highlight request:\n{:?}", params))
      .await;

    Ok(None)
  }

  async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
    Ok(())
  }
}

#[tokio::main]
async fn main() {
  let stdin = tokio::io::stdin();
  let stdout = tokio::io::stdout();

  let (service, socket) = LspService::new(|client| Backend { client });

  Server::new(stdin, stdout, socket).serve(service).await;
}
