import * as vscode from "vscode";
import { LanguageClient, TransportKind } from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
  const lspPath = vscode.workspace.getConfiguration("moonjuice").get("lspPath");

  if (typeof lspPath !== "string" || lspPath.length < 1) {
    return vscode.window.showErrorMessage("The `#moonjuice.lspPath#` setting was not configured");
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

  const disposable = vscode.commands.registerCommand("moonjuice-vscode.helloWorld", () => {
    return vscode.window.showInformationMessage(vscode.workspace.getConfiguration("moonjuice").get("lspPath") ?? "");
  });

  context.subscriptions.push(disposable);
}

export async function deactivate() {
  await client?.dispose();
  client = undefined;
}
