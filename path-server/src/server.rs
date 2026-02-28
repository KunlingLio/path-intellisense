use std::sync::Arc;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types;

use crate::resolver::PathResolver;

#[derive(Debug)]
pub struct PathServer {
    client: tower_lsp::Client,
    resolver: Arc<PathResolver>,
}

impl PathServer {
    pub fn new(client: tower_lsp::Client) -> Self {
        Self {
            client,
            resolver: Arc::new(PathResolver::new()),
        }
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for PathServer {
    async fn initialize(&self, params: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult> {
        if let Some(url) = params.root_uri {
            self.client
                .log_message(lsp_types::MessageType::INFO, format!("[Path Server] Workspace root: {}", url))
                .await;
            self.resolver.add_workspace_root(&url);
        }
        Ok(lsp_types::InitializeResult {
            capabilities: lsp_types::ServerCapabilities {
                completion_provider: Some(lsp_types::CompletionOptions {
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        "/".to_string(),
                        "\\".to_string(), // For windows paths
                        ":".to_string(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                workspace: Some(lsp_types::WorkspaceServerCapabilities {
                    workspace_folders: Some(lsp_types::WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: None,
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: lsp_types::InitializedParams) {
        self.client
            .log_message(lsp_types::MessageType::INFO, "[Path Server] Initialized!")
            .await;
    }

    async fn did_change_workspace_folders(&self, params: lsp_types::DidChangeWorkspaceFoldersParams) {
        for folder in params.event.added {
            self.client
                .log_message(lsp_types::MessageType::INFO, format!("[Path Server] Added workspace folder: {}", folder.uri))
                .await;
            self.resolver.add_workspace_root(&folder.uri);
        }
        for folder in params.event.removed {
            self.client
                .log_message(lsp_types::MessageType::INFO, format!("[Path Server] Removed workspace folder: {}", folder.uri))
                .await;
            self.resolver.remove_workspace_root(&folder.uri);
        }
    }

    async fn completion(&self, params: lsp_types::CompletionParams) -> Result<Option<lsp_types::CompletionResponse>> {
        let completion_items = self.resolver.complete("").await;
        let completion_items = completion_items.into_iter().map(|path| {
            lsp_types::CompletionItem {
                label: path.to_string_lossy().to_string(),
                kind: if path.is_dir() {
                    Some(lsp_types::CompletionItemKind::FOLDER)
                } else {
                    Some(lsp_types::CompletionItemKind::FILE)
                },
                ..Default::default()
            }
        }).collect::<Vec<_>>();
        return Ok(Some(lsp_types::CompletionResponse::Array(completion_items)));
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
