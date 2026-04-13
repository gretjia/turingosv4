// Tier 2: Librarian — compresses tape into agent memory (Engine 4: Speciation)
// Constitutional basis: Law 3 (Digital Property Rights — per-agent skill path)
// V3L-49: Lamarckian hallucination: group DNA > individual learning

use crate::sdk::tool::TuringTool;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// The Librarian compresses the group tape into per-agent memory files.
///
/// Design principle (V3L-49): Individual "learning" is post-hoc rationalization.
/// Group DNA (success/failure logs) is the actual ground truth.
/// The Librarian bridges these — reading from immutable logs, writing to
/// each agent's learned.md as a falsifiable theory (logs win on conflict).
pub struct LibrarianTool {
    skills_dir: PathBuf,
    compress_interval: usize,
    append_count: usize,
}

/// Compression request data for an external LLM call.
#[derive(Debug, Serialize)]
pub struct CompressionPrompt {
    pub successes: Vec<String>,
    pub failures: Vec<String>,
    pub rejection_categories: HashMap<String, usize>,
}

impl LibrarianTool {
    pub fn new(skills_dir: &str, compress_interval: usize) -> Self {
        LibrarianTool {
            skills_dir: PathBuf::from(skills_dir),
            compress_interval,
            append_count: 0,
        }
    }

    /// Check if compression should fire (every N appends).
    pub fn should_compress(&self) -> bool {
        self.compress_interval > 0 && self.append_count > 0
            && self.append_count % self.compress_interval == 0
    }

    /// Build a compression prompt from success/failure data.
    /// The actual LLM call happens externally (engine separation).
    pub fn build_compression_prompt(
        &self,
        successes: Vec<String>,
        failures: Vec<String>,
        rejection_categories: HashMap<String, usize>,
    ) -> CompressionPrompt {
        CompressionPrompt {
            successes,
            failures,
            rejection_categories,
        }
    }

    /// Write compressed memory to an agent's skill file.
    pub fn write_agent_memory(&self, agent_id: &str, memory: &str) -> Result<(), std::io::Error> {
        let agent_dir = self.skills_dir.join(agent_id);
        std::fs::create_dir_all(&agent_dir)?;
        let path = agent_dir.join("learned.md");
        std::fs::write(&path, memory)
    }

    /// Read an agent's current skill file.
    pub fn read_agent_memory(&self, agent_id: &str) -> Option<String> {
        let path = self.skills_dir.join(agent_id).join("learned.md");
        std::fs::read_to_string(&path).ok()
    }
}

impl TuringTool for LibrarianTool {
    fn manifest(&self) -> &str {
        "librarian"
    }

    fn on_post_append(&mut self, _author: &str, _node_id: &str) {
        self.append_count += 1;
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_interval() {
        let mut lib = LibrarianTool::new("/tmp/test_skills", 8);
        for _ in 0..7 {
            lib.on_post_append("A0", "n1");
            assert!(!lib.should_compress());
        }
        lib.on_post_append("A0", "n8");
        assert!(lib.should_compress());
    }

    #[test]
    fn test_build_compression_prompt() {
        let lib = LibrarianTool::new("/tmp/test_skills", 8);
        let prompt = lib.build_compression_prompt(
            vec!["success_1".into()],
            vec!["failure_1".into()],
            HashMap::from([("parse_error".into(), 3)]),
        );
        assert_eq!(prompt.successes.len(), 1);
        assert_eq!(prompt.failures.len(), 1);
        assert_eq!(*prompt.rejection_categories.get("parse_error").unwrap(), 3);
    }

    #[test]
    fn test_zero_interval_never_compresses() {
        let mut lib = LibrarianTool::new("/tmp/test_skills", 0);
        for _ in 0..100 {
            lib.on_post_append("A0", "n1");
        }
        assert!(!lib.should_compress());
    }
}
