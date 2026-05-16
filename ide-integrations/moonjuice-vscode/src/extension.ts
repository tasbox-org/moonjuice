import * as vscode from "vscode";

export function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand("moonjuice-vscode.helloWorld", () => {
    return vscode.window.showInformationMessage(vscode.workspace.getConfiguration("moonjuice").get("lspPath") ?? "");
  });

  context.subscriptions.push(disposable);
}

export function deactivate() {}
