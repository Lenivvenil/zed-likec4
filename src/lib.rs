use std::{env, fs};
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

const PACKAGE_NAME: &str = "@likec4/lsp";
const LANGUAGE_SERVER_ID: &str = "likec4-language-server";

struct LikeC4Extension {
    did_find_server: bool,
}

impl LikeC4Extension {
    fn server_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let (os, _arch) = zed::current_platform();
        let server_path = match os {
            zed::Os::Mac | zed::Os::Linux => "node_modules/.bin/likec4-lsp".to_string(),
            zed::Os::Windows => "node_modules/@likec4/lsp/bin/likec4-lsp.mjs".to_string(),
        };

        if self.did_find_server && self.server_exists(&server_path) {
            return Ok(server_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = match zed::npm_package_latest_version(PACKAGE_NAME) {
            Ok(version) => Some(version),
            Err(_) if self.server_exists(&server_path) => None,
            Err(err) => return Err(err),
        };

        let needs_install = if let Some(ref latest) = latest_version {
            !self.server_exists(&server_path)
                || zed::npm_package_installed_version(PACKAGE_NAME)
                    .ok()
                    .flatten()
                    .is_none_or(|installed| installed != *latest)
        } else {
            false
        };

        if let (true, Some(ref latest)) = (needs_install, &latest_version) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(PACKAGE_NAME, latest);
            match result {
                Ok(()) => {
                    if !self.server_exists(&server_path) {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{server_path}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists(&server_path) {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(server_path)
    }
}

impl zed::Extension for LikeC4Extension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        let server_abs = env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_abs, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .map(|s| s.initialization_options.clone())
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree).map(|s| s.settings.clone())
    }
}

zed::register_extension!(LikeC4Extension);
