use std::fs;
use zed_extension_api::{
    self as zed, lsp::Completion, CodeLabel, CodeLabelSpan, LanguageServerId,
    LanguageServerInstallationStatus as Status, Result, Worktree,
};

struct MesonExtension {
    cached_path: Option<String>,
}

impl MesonExtension {
    fn lsp_path(&mut self, id: &LanguageServerId, tree: &Worktree) -> Result<String> {
        if let Some(path) = tree.which("mesonlsp") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_path {
            if fs::metadata(path).map_or(false, |s| s.is_file()) {
                return Ok(path.clone());
            }
        }

        let (platform, arch) = zed::current_platform();
        zed::set_language_server_installation_status(&id, &Status::CheckingForUpdate);

        let asset = format!(
            "mesonlsp-{arch}-{os}.zip",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 => "x86",
            },
            os = match platform {
                zed::Os::Linux => "unknown-linux-musl",
                zed::Os::Windows => "pc-windows-gnu",
                zed::Os::Mac => "apple-darwin",
            },
        );

        let dir_name = asset.clone();
        let bin_path = format!("{dir_name}/mesonlsp");

        if !fs::metadata(&bin_path).map_or(false, |s| s.is_file()) {
            zed::set_language_server_installation_status(&id, &Status::Downloading);

            let release = zed::latest_github_release(
                "jcwasmx86/mesonlsp",
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;

            let asset_dl = release
                .assets
                .iter()
                .find(|a| a.name == asset)
                .ok_or_else(|| format!("no asset found matching {:?}", asset))?;
            zed::download_file(
                &asset_dl.download_url,
                &dir_name,
                zed::DownloadedFileType::Zip,
            )?;

            for entry in fs::read_dir(".").map_err(|e| format!("failed to read dir: {e}"))? {
                let entry = entry.map_err(|e| format!("failed to load directory: {e}"))?;
                if entry.file_name().to_str() != Some(&dir_name) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_path = Some(bin_path.clone());
        Ok(bin_path)
    }
}

impl zed::Extension for MesonExtension {
    fn new() -> Self {
        Self { cached_path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.lsp_path(language_server_id, worktree)?,
            args: vec!["--lsp".to_string()],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<zed::CodeLabel> {
        let kind = match completion.kind {
            Some(zed::lsp::CompletionKind::Function) | Some(zed::lsp::CompletionKind::Method) => {
                match completion.detail {
                    Some(a) => a,
                    _ => completion.label,
                }
            }
            _ => match completion.detail {
                Some(a) => format!("{} {}", completion.label, a),
                _ => completion.label,
            },
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..kind.len())],
            filter_range: (0..kind.len()).into(),
            code: kind,
        })
    }
}

zed::register_extension!(MesonExtension);
