use std::sync::RwLock;
use std::path::PathBuf;
use std::collections::HashSet;

use tower_lsp::lsp_types::{self, Url};

#[derive(Debug)]
pub struct PathResolver {
    workspace_root: RwLock<HashSet<lsp_types::Url>>,
}

impl PathResolver {
    pub fn new() -> Self {
        PathResolver { workspace_root: RwLock::new(HashSet::new()) }
    }

    pub fn add_workspace_root(&self, url: &Url) {
        let mut roots = self.workspace_root.write().unwrap();
        roots.insert(url.clone());
    }

    pub fn remove_workspace_root(&self, url: &Url) {
        let mut roots = self.workspace_root.write().unwrap();
        roots.remove(url);
    }

    pub async fn complete(&self, input: &str) -> Vec<PathBuf> {
        // For demonstration purposes, we return a static list of paths.
        // In a real implementation, you would perform actual file system operations here.
        vec![
            PathBuf::from("example.txt"),
            PathBuf::from("src"),
            PathBuf::from("README.md"),
        ]
    }
}
