# Avalonia AXAML for Zed

AXAML syntax highlighting, completion, and diagnostics through the [AXSG `axsg-lsp` language server](https://github.com/wieslawsoltes/XamlToCSharpGenerator). This extension does not provide an embedded preview panel.

## Requirements and language server installation

- Zed with extension support.
- The .NET 10 runtime, available as `dotnet` on Zed's `PATH`, to run the downloaded language server. The SDK is not required by the extension.
- Network access to GitHub Releases on first launch when the server is not already installed.

When an `.axaml` file is opened, the extension first uses an `axsg-lsp` executable already on Zed's `PATH`. Otherwise, it downloads the official `XamlToCSharpGenerator.LanguageServer.Tool` version `0.1.0-alpha.28` release package into Zed's extension work directory and launches its DLL with `dotnet`. This is a pinned prerelease version of AXSG; the extension's version is independent of AXSG's version. The tool is downloaded on first use, not when the extension itself is installed.

If you prefer to manage the server yourself, install it and ensure Zed can find `axsg-lsp`:

```sh
dotnet tool install --global XamlToCSharpGenerator.LanguageServer.Tool --version 0.1.0-alpha.28
command -v axsg-lsp
```

On macOS and Linux, the global tool directory is usually `~/.dotnet/tools`. Zed may have a different `PATH` from your terminal.

## Install locally

Open Zed's command palette, run `zed: install dev extension`, and select this `lsp/zed-avalonia` directory. Open an Avalonia project and an `.axaml` file. Zed should select **AXAML** as the language and start **AXSG AXAML Language Server**. The first download may take longer; subsequent starts reuse the installed package. Use `dev: open language server logs` or `zed: open log` to inspect startup failures.

For LSP semantic highlighting in addition to the bundled XML syntax highlighting, set `"semantic_tokens": "combined"` in Zed settings. Zed defaults to semantic tokens off.

## Publication

The extension is submitted to the [Zed extensions registry](https://zed.dev/docs/extensions/publishing/publishing-guide). It becomes installable from Zed's gallery after the registry PR is merged and the extension is published. Verify a fresh installation without `axsg-lsp` on `PATH` when testing the managed download path.
