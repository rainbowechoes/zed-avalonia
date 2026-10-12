use zed_extension_api as zed;

const SERVER_ID: &str = "axsg-lsp";
const TOOL_VERSION: &str = "0.1.0-alpha.28";
const TOOL_PACKAGE: &str = "XamlToCSharpGenerator.LanguageServer.Tool";
const SERVER_DLL: &str = "tools/net10.0/any/XamlToCSharpGenerator.LanguageServer.dll";

struct AvaloniaExtension;

fn managed_server_path() -> Result<std::path::PathBuf, String> {
    std::env::current_dir()
        .map(|dir| {
            dir.join(format!("axsg-lsp-package-{TOOL_VERSION}"))
                .join(SERVER_DLL)
        })
        .map_err(|error| format!("Cannot locate the extension work directory: {error}"))
}

fn install_server(
    language_server_id: &zed::LanguageServerId,
    server_path: &std::path::Path,
) -> Result<(), String> {
    let install_dir = format!("axsg-lsp-package-{TOOL_VERSION}");
    let url = format!(
        "https://github.com/wieslawsoltes/XamlToCSharpGenerator/releases/download/v{TOOL_VERSION}/{TOOL_PACKAGE}.{TOOL_VERSION}.nupkg"
    );

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    zed::download_file(&url, &install_dir, zed::DownloadedFileType::Zip)
        .map_err(|error| format!("Failed to download AXSG {TOOL_VERSION}: {error}"))?;

    if !server_path.is_file() {
        return Err(format!(
            "AXSG {TOOL_VERSION} download did not contain the language server"
        ));
    }

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::None,
    );
    Ok(())
}

impl zed::Extension for AvaloniaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        let command = if let Some(command) = worktree.which(SERVER_ID) {
            zed::Command {
                command,
                args: Vec::new(),
                env: Vec::new(),
            }
        } else {
            let dotnet = worktree.which("dotnet").ok_or_else(|| {
                "AXSG requires the .NET 10 runtime. Install it and restart Zed, or put axsg-lsp on Zed's PATH."
                    .to_string()
            })?;
            let server_path = managed_server_path()?;
            if !server_path.is_file() {
                if let Err(error) = install_server(language_server_id, &server_path) {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed::LanguageServerInstallationStatus::Failed(error.clone()),
                    );
                    return Err(error);
                }
            }
            zed::Command {
                command: dotnet,
                args: vec![server_path.to_string_lossy().into_owned()],
                env: Vec::new(),
            }
        };

        Ok(command)
    }
}

zed::register_extension!(AvaloniaExtension);
