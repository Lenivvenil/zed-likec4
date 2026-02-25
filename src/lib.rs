use std::{env, fs};
use zed_extension_api::{
    self as zed,
    serde_json,
    settings::LspSettings,
    Result,
};

const PACKAGE_NAME: &str = "@likec4/language-server";
const LANGUAGE_SERVER_ID: &str = "likec4-language-server";

/// Custom ESM resolve hook passed to Node.js via `--import data:text/javascript,...`.
///
/// Fixes two issues with the `@likec4/language-server` package under Node.js v22+:
///
/// 1. **Missing `.js` extensions** — `vscode-languageserver` has no `exports` field in its
///    package.json, so ESM subpath imports like `vscode-languageserver/node` fail under strict
///    ESM resolution. The hook retries with a `.js` suffix.
///
/// 2. **Missing optional peer dependencies** — `bundle-require` and `esbuild` are optional
///    peer deps used only for loading `likec4.config.ts` files. They are not needed for LSP
///    operations, so the hook provides lightweight stubs instead of requiring heavy native
///    binaries.
fn esm_resolve_hook() -> String {
    let mocks = r#"{"bundle-require":"export function bundleRequire(){throw new Error('not available')}","esbuild":"export function formatMessagesSync(){return[]}"}"#;

    let loader = format!(
        concat!(
            "const M={mocks};",
            "export async function resolve(s,c,n){{",
            "if(M[s])return{{url:'data:text/javascript,'+encodeURIComponent(M[s]),shortCircuit:true}};",
            "try{{return await n(s,c)}}",
            "catch(e){{",
            "if(e.code==='ERR_MODULE_NOT_FOUND'&&s.includes('/')&&!s.endsWith('.js')&&!s.startsWith('node:')&&!s.startsWith('.')){{",
            "try{{return await n(s+'.js',c)}}catch{{}}}};",
            "throw e}}}}",
        ),
        mocks = mocks,
    );

    let hook = format!(
        "import{{register}}from'node:module';register('data:text/javascript,{loader}',import.meta.url);",
        loader = percent_encode(&loader),
    );

    format!("data:text/javascript,{}", percent_encode(&hook))
}

/// Percent-encode a string for use in a data: URI.
fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~'
            | b'!' | b'*' | b'(' | b')' => {
                out.push(byte as char);
            }
            _ => {
                out.push('%');
                out.push(char::from(HEX[(byte >> 4) as usize]));
                out.push(char::from(HEX[(byte & 0xF) as usize]));
            }
        }
    }
    out
}

const HEX: [u8; 16] = *b"0123456789ABCDEF";

struct LikeC4Extension {
    did_find_server: bool,
}

impl LikeC4Extension {
    fn server_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        let (os, _arch) = zed::current_platform();
        let server_path = match os {
            zed::Os::Mac | zed::Os::Linux => {
                "node_modules/.bin/likec4-language-server".to_string()
            }
            zed::Os::Windows => {
                "node_modules/@likec4/language-server/bin/likec4-language-server.mjs".to_string()
            }
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
                || zed::npm_package_installed_version(PACKAGE_NAME)?
                    .is_none_or(|installed| installed != *latest)
        } else {
            false
        };

        if needs_install {
            let latest = latest_version.as_ref().unwrap();

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
            args: vec![
                "--import".to_string(),
                esm_resolve_hook(),
                server_abs,
                "--stdio".to_string(),
            ],
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
        LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .map(|s| s.settings.clone())
    }
}

zed::register_extension!(LikeC4Extension);
