# LikeC4 for Zed

[LikeC4](https://likec4.dev) language support for the [Zed](https://zed.dev) editor.

LikeC4 is an architecture-as-code DSL that lets you describe your software architecture with a simple, human-readable language and visualize it as interactive diagrams.

This extension brings first-class LikeC4 editing experience to Zed with a custom [tree-sitter grammar](https://github.com/Lenivvenil/tree-sitter-likec4) and the official [LikeC4 Language Server](https://www.npmjs.com/package/@likec4/language-server).

## Features

- **Syntax highlighting** — rich, tree-sitter-based highlighting for all LikeC4 constructs
- **Code completion** — context-aware suggestions for elements, relationships, views, and keywords
- **Diagnostics** — real-time validation errors and warnings as you type
- **Hover information** — documentation and details on hover
- **Go to Definition** — jump to element and relationship definitions
- **Find References** — locate all usages of an element across your workspace
- **Rename** — safely rename elements with all references updated
- **Code outline** — navigate your architecture models with the Outline panel
- **Auto-indentation** — smart indentation for all block types
- **Bracket matching** — automatic matching of braces

## Installation

1. Open Zed
2. Open the Extensions panel (`Cmd+Shift+X` on macOS)
3. Search for "LikeC4"
4. Click **Install**

The extension will automatically download and manage the LikeC4 language server.

## Configuration

### Language Server Settings

You can pass custom settings to the LikeC4 language server through your Zed settings (`Cmd+,`):

```json
{
  "lsp": {
    "likec4-language-server": {
      "settings": {},
      "initialization_options": {}
    }
  }
}
```

## Limitations

- **No diagram preview** — Zed does not currently support webview panels in extensions, so live diagram visualization is not available. Use the [LikeC4 CLI](https://likec4.dev/tooling/cli/) or the [VSCode extension](https://likec4.dev/tooling/vscode/) for diagram previews.

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (installed via rustup)
- The `wasm32-wasip1` target:
  ```bash
  rustup target add wasm32-wasip1
  ```

### Building

```bash
cargo check --target wasm32-wasip1
```

### Testing locally

1. Open Zed
2. Open the Extensions panel
3. Click **Install Dev Extension**
4. Select the root directory of this repository
5. Open a `.likec4` file to verify syntax highlighting and the language server

### Resources

- [Zed Extension Development Guide](https://zed.dev/docs/extensions/developing-extensions)
- [Zed Extension API Reference](https://docs.rs/zed_extension_api)

## Links

- [LikeC4 Website](https://likec4.dev)
- [LikeC4 DSL Specification](https://likec4.dev/dsl/specification/)
- [LikeC4 Language Server (npm)](https://www.npmjs.com/package/@likec4/language-server)
- [LikeC4 GitHub Repository](https://github.com/likec4/likec4)
- [Tree-sitter Grammar](https://github.com/Lenivvenil/tree-sitter-likec4)

## License

MIT — see [LICENSE](LICENSE) for details.
