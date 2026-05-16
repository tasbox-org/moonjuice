# MoonJuice VSCode Language Support

Language support for the MoonJuice scripting language.

## Features

> IMPORTANT: This extension is still work-in-progress, language support is fairly rudimentary

Syntax highlighting and basic editing capabilities (bracket/quote auto-closing, auto-indenting):

![demonstration of syntax highlighting and basic editing features](images/basic.gif)

## Requirements

The MoonJuice language server:

- `cargo install moonjuice-lsp` (ensure `~/.cargo/bin` is in `PATH`)
  - Preferred installation method for easy updates
- WIP: Instructions for installing manually

## Extension Settings

This extension contributes the following settings:

* `moonjuice.lspPath`: Path to the MoonJuice language server (defaults to reading `moonjuice-lsp` from `PATH`)

## Release Notes

### 0.1.0

Preview version of MoonJuice language support with basic LSP integration for semantic highlighting.
