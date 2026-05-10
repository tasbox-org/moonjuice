use tower_lsp_server::ls_types::{InitializeParams, InitializeResult};
use tower_lsp_server::{LanguageServer, LspService};

#[derive(Debug)]
struct Backend {}

impl LanguageServer for Backend {
  async fn initialize(&self, params: InitializeParams) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
    Ok(InitializeResult::default())
  }

  async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
    Ok(())
  }
}

fn main() {
  let lsp_service = LspService::new(|_client| Backend {});
  dbg!(lsp_service);
}
