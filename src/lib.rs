use zed_extension_api as zed;

struct PathIntellisenseExtension;

impl zed::Extension for PathIntellisenseExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "path-lsp-server".to_string(),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(PathIntellisenseExtension);
