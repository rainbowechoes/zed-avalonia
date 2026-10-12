use zed_extension_api as zed;

const SERVER_ID: &str = "axsg-lsp";
const TOOL_PACKAGE: &str = "XamlToCSharpGenerator.LanguageServer.Tool";
const TOOL_VERSION: &str = "0.1.0-alpha.28";

struct AvaloniaExtension;

fn managed_server_path() -> Result<std::path::PathBuf, String> {
    let executable = if zed::current_platform().0 == zed::Os::Windows {
        "axsg-lsp.exe"
    } else {
        "axsg-lsp"
    };

    std::env::current_dir()
        .map(|dir| {
            dir.join(format!("axsg-lsp-{TOOL_VERSION}"))
                .join(executable)
        })
        .map_err(|error| format!("Cannot locate the extension work directory: {error}"))
}

fn install_server(
    language_server_id: &zed::LanguageServerId,
    dotnet: &str,
    executable: &std::path::Path,
) -> Result<(), String> {
    let install_dir = executable
        .parent()
        .ok_or_else(|| "Cannot locate the AXSG installation directory".to_string())?;
    let work_dir = install_dir
        .parent()
        .ok_or_else(|| "Cannot locate the extension work directory".to_string())?;

    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    let output = zed::process::Command::new(dotnet)
        .args([
            "tool",
            "install",
            TOOL_PACKAGE,
            "--version",
            TOOL_VERSION,
            "--tool-path",
            &install_dir.to_string_lossy(),
        ])
        .env("DOTNET_CLI_HOME", work_dir.to_string_lossy())
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .env(
            "NUGET_PACKAGES",
            work_dir.join("nuget-packages").to_string_lossy(),
        )
        .env(
            "NUGET_HTTP_CACHE_PATH",
            work_dir.join("nuget-http-cache").to_string_lossy(),
        )
        .output()
        .map_err(|error| format!("Failed to run dotnet tool install: {error}"))?;

    if output.status != Some(0) || !executable.is_file() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "AXSG {TOOL_VERSION} installation failed: {}",
            if stderr.trim().is_empty() {
                stdout.trim()
            } else {
                stderr.trim()
            }
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
            command
        } else {
            let executable = managed_server_path()?;
            if !executable.is_file() {
                let dotnet = worktree.which("dotnet").ok_or_else(|| {
                    "AXSG requires the .NET 10 SDK. Install it and restart Zed, or put axsg-lsp on Zed's PATH."
                        .to_string()
                })?;

                if let Err(error) = install_server(language_server_id, &dotnet, &executable) {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed::LanguageServerInstallationStatus::Failed(error.clone()),
                    );
                    return Err(error);
                }
            }
            executable.to_string_lossy().into_owned()
        };

        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(AvaloniaExtension);
