// Tier 2: Free SearchTool — zero-cost information retrieval
// Constitutional basis: Law 1 (Information is Free — search costs nothing)

use crate::sdk::tool::{ToolSignal, TuringTool};
use std::any::Any;
use std::path::Path;

/// Zero-cost search tool.
/// Law 1: "Thinking and inquiry is free."
/// Agents can search without spending any Coins.
pub struct SearchTool {
    pub search_paths: Vec<String>,
    pub max_results: usize,
}

impl SearchTool {
    pub fn new(search_paths: Vec<String>, max_results: usize) -> Self {
        SearchTool {
            search_paths,
            max_results,
        }
    }

    /// Search for files matching a query string.
    /// Sanitizes query to prevent injection.
    /// Returns list of matching file paths (no cost, no tape write).
    pub fn search(&self, raw_query: &str) -> Vec<String> {
        let query = sanitize_query(raw_query);
        if query.is_empty() {
            return vec![];
        }

        let mut results = Vec::new();

        for search_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.to_lowercase().contains(&query.to_lowercase()) {
                                results.push(path.display().to_string());
                            }
                        }
                    }
                    if results.len() >= self.max_results {
                        break;
                    }
                }
            }
        }

        results
    }
}

/// Sanitize search query: allow only alphanumeric, underscore, dot, space.
fn sanitize_query(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '.' || *c == ' ' || *c == '\'')
        .collect()
}

impl TuringTool for SearchTool {
    fn manifest(&self) -> &str {
        "search"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_query() {
        assert_eq!(sanitize_query("hello world"), "hello world");
        assert_eq!(sanitize_query("rm -rf /"), "rm rf ");
        assert_eq!(sanitize_query("test_file.lean"), "test_file.lean");
        assert_eq!(sanitize_query("$(evil)"), "evil");
    }

    #[test]
    fn test_search_empty_query() {
        let tool = SearchTool::new(vec![], 10);
        assert!(tool.search("").is_empty());
    }

    #[test]
    fn test_search_nonexistent_path() {
        let tool = SearchTool::new(vec!["/nonexistent/path".into()], 10);
        assert!(tool.search("test").is_empty());
    }
}
