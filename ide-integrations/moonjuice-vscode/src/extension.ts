import * as vscode from "vscode";
import { LanguageClient, TransportKind } from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand("moonjuice.restartLsp", async () => {
    await client?.restart();
    await vscode.window.showInformationMessage("MoonJuice Language Server restarted");
  });

  context.subscriptions.push(disposable);

  const lspPath = vscode.workspace.getConfiguration("moonjuice.lsp").get("path");

  if (typeof lspPath !== "string" || lspPath.length < 1) {
    return vscode.window.showErrorMessage("The `#moonjuice.lsp.path#` setting was not configured");
  }

  client = new LanguageClient(
    "moonjuice",
    "MoonJuice",
    {
      command: lspPath,
      transport: TransportKind.stdio,
    },
    {
      documentSelector: [{ scheme: "file", language: "moonjuice" }],
    },
  );

  await client.start();
}

export async function deactivate() {
  await client?.dispose();
  client = undefined;
}
