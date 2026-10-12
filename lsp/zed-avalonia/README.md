# Avalonia AXAML for Zed

Version 1.0.0 adds AXAML syntax highlighting, completion, and diagnostics through the [AXSG `axsg-lsp` language server](https://github.com/wieslawsoltes/XamlToCSharpGenerator). It does not provide an embedded preview panel.

## Requirements and language server installation

- Zed with extension support.
- The .NET 10 SDK, available as `dotnet` on Zed's `PATH`, for automatic language server installation. The server itself requires the .NET 10 runtime.
- Network access to NuGet on first launch when the server is not already installed.

When an `.axaml` file is opened, the extension first uses an `axsg-lsp` executable already on Zed's `PATH`. Otherwise, it installs `XamlToCSharpGenerator.LanguageServer.Tool` version `0.1.0-alpha.28` into Zed's extension work directory, then launches the installed executable. This is a pinned prerelease version of AXSG; the extension's 1.0.0 version is independent of AXSG's version. The tool is installed on first use, not when the extension itself is installed.

If you prefer to manage the server yourself, install it and ensure Zed can find `axsg-lsp`:

```sh
dotnet tool install --global XamlToCSharpGenerator.LanguageServer.Tool --version 0.1.0-alpha.28
command -v axsg-lsp
```

On macOS and Linux, the global tool directory is usually `~/.dotnet/tools`. Zed may have a different `PATH` from your terminal.

## Install locally

Open Zed's command palette, run `zed: install dev extension`, and select this `lsp/zed-avalonia` directory. Open an Avalonia project and an `.axaml` file. Zed should select **AXAML** as the language and start **AXSG AXAML Language Server**. The first managed installation may take longer; subsequent starts reuse the installed tool. Use `dev: open language server logs` or `zed: open log` to inspect startup failures.

For LSP semantic highlighting in addition to the bundled XML syntax highlighting, set `"semantic_tokens": "combined"` in Zed settings. Zed defaults to semantic tokens off.

## Publication

This directory is ready for local 1.0.0 testing. A public release through Zed's extension gallery additionally requires publishing this directory in a public Git repository and submitting it to the [Zed extensions registry](https://zed.dev/docs/extensions/publishing/publishing-guide). The registry, not the version number alone, makes the extension installable from Zed's gallery. Verify a fresh installation without `axsg-lsp` on `PATH` before submission.
