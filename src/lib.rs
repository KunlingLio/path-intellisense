use zed_extension_api::{self as zed, LanguageServerId, Result};

#[derive(Default)]
struct CSpellExtension {
}

impl zed::Extension for CSpellExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "/Users/lkl/Code/path-intellisense/target/debug/path-server".to_string(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CSpellExtension);
