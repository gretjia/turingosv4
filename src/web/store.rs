/// TRACE_MATRIX FC1-N5: in-memory task store for Phase 7 W4.2.
///
/// Provides a small, append-only ring store for tasks created via
/// POST /api/task/open.  This is NOT ChainTape-backed — entries are lost on
/// backend restart.  That is acceptable for Phase 7's research scope per
/// §6 partial-witness allowance; real durability is Phase 8+ work.
///
/// # Cap mechanism
///
/// After every `push`, if `entries.len() > 1000`, the **oldest 500** entries
/// are dropped (the front of the Vec) so the Vec never grows beyond 1000.
/// This prevents unbounded memory growth under continuous test or load.
///
/// # Concurrency
///
/// Uses `std::sync::Mutex` (not `tokio::sync::Mutex`).  The critical section
/// is a Vec push/clone that takes microseconds; no `.await` is ever called
/// inside the lock.
#[cfg(feature = "web")]
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// TRACE_MATRIX FC1-N5: a single in-memory record for a task created via the
/// write path.
///
/// Fields map directly to `WsBroadcastMsg::TaskCreated` plus a wall-clock
/// timestamp so callers can order entries by creation time.
#[cfg(feature = "web")]
#[derive(Debug, Clone)]
pub(crate) struct TaskEntry {
    /// Canonical task ID returned/parsed from the CLI shellout.
    pub(crate) task_id: String,
    /// Agent ID supplied in the POST body.
    pub(crate) agent_id: String,
    /// Problem identifier supplied in the POST body.
    pub(crate) problem_id: String,
    /// Bounty in μCoin supplied in the POST body.
    pub(crate) bounty: u64,
    /// Unix timestamp (seconds since UNIX_EPOCH) at push time.
    pub(crate) created_at_unix: u64,
}

/// TRACE_MATRIX FC1-N5: in-memory append-only store for tasks created during
/// this server process lifetime.
///
/// Capped at 1 000 entries: when `push` would push past the cap, the oldest
/// 500 entries are evicted, keeping the newest 500 plus the new entry (≤ 1001
/// entries momentarily, then trimmed to 501).  The cap prevents unbounded
/// memory growth during long test runs or high POST volume.
///
/// Designed for shared ownership via `std::sync::Arc<TaskMemoryStore>`.
#[cfg(feature = "web")]
pub(crate) struct TaskMemoryStore {
    entries: Mutex<Vec<TaskEntry>>,
}

#[cfg(feature = "web")]
impl TaskMemoryStore {
    /// Create a new, empty store.
    ///
    /// TRACE_MATRIX FC2-N16: Phase 7 web — TaskMemoryStore::new (AppState init).
    pub(crate) fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    /// Append `entry` to the store.
    ///
    /// If the store exceeds 1 000 entries after the push, the oldest 500
    /// entries are dropped.  This is the only write operation; no deduplication
    /// is performed.
    ///
    /// TRACE_MATRIX FC2-N16: Phase 7 web — TaskMemoryStore::push (bounded FIFO).
    pub(crate) fn push(&self, entry: TaskEntry) {
        let mut guard = self.entries.lock().expect("TaskMemoryStore mutex poisoned");
        guard.push(entry);
        if guard.len() > 1000 {
            // Drop the oldest 500 entries (the front of the Vec).
            guard.drain(..500);
        }
    }

    /// Return a cloned snapshot of the current entries.
    ///
    /// Entries are returned in insertion order (oldest first); callers that
    /// want newest-first should reverse the result.
    ///
    /// TRACE_MATRIX FC2-N16: Phase 7 web — TaskMemoryStore::snapshot (read-view).
    pub(crate) fn snapshot(&self) -> Vec<TaskEntry> {
        self.entries
            .lock()
            .expect("TaskMemoryStore mutex poisoned")
            .clone()
    }

    /// Return the current number of stored entries.
    ///
    /// Primarily used in tests to assert cap behaviour.
    ///
    /// TRACE_MATRIX FC2-N16: Phase 7 web — TaskMemoryStore::len (test cap assertion).
    pub(crate) fn len(&self) -> usize {
        self.entries
            .lock()
            .expect("TaskMemoryStore mutex poisoned")
            .len()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(all(feature = "web", test))]
mod tests {
    use super::*;

    fn make_entry(task_id: &str) -> TaskEntry {
        TaskEntry {
            task_id: task_id.to_string(),
            agent_id: "agent_test".to_string(),
            problem_id: "prob_test".to_string(),
            bounty: 1_000,
            created_at_unix: 0,
        }
    }

    #[test]
    fn push_and_snapshot_round_trip() {
        let store = TaskMemoryStore::new();
        store.push(make_entry("t_001"));
        store.push(make_entry("t_002"));
        let snap = store.snapshot();
        assert_eq!(snap.len(), 2);
        assert_eq!(snap[0].task_id, "t_001");
        assert_eq!(snap[1].task_id, "t_002");
    }

    #[test]
    fn len_reflects_entry_count() {
        let store = TaskMemoryStore::new();
        assert_eq!(store.len(), 0);
        store.push(make_entry("t_a"));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn cap_kicks_in_at_1001_entries() {
        let store = TaskMemoryStore::new();
        for i in 0..1001 {
            store.push(make_entry(&format!("t_{i:04}")));
        }
        // After 1001 pushes: push the 1001st triggers drain(..500), leaving 501.
        assert!(
            store.len() <= 1000,
            "store must cap at 1000; got {}",
            store.len()
        );
    }

    #[test]
    fn cap_keeps_newest_entries() {
        let store = TaskMemoryStore::new();
        // Push 1001 entries; after cap the oldest 500 should be gone.
        for i in 0..1001 {
            store.push(make_entry(&format!("t_{i:04}")));
        }
        let snap = store.snapshot();
        // The first surviving entry should be t_0500 (index 500).
        assert_eq!(
            snap[0].task_id, "t_0500",
            "oldest surviving entry should be t_0500; got {:?}",
            snap[0].task_id
        );
        // The last entry should be the 1001st push (t_1000).
        assert_eq!(
            snap.last().unwrap().task_id,
            "t_1000",
            "newest entry should be t_1000"
        );
    }
}
