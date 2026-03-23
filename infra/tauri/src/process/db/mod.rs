pub mod execution_process_logs;

pub use execution_process_logs::{ExecutionProcessLogs, init_db_pool};

use std::fs;
use std::path::PathBuf;

/// Resolve SQLite database path.
///
/// Priority:
/// 1. `DB_PATH` env var
/// 2. `~/.hivelaunch/hivelaunch.db`
/// 3. `<temp>/.hivelaunch/hivelaunch.db` (when home directory is unavailable)
pub fn resolve_db_path() -> PathBuf {
    if let Ok(path) = std::env::var("DB_PATH") {
        return PathBuf::from(path);
    }

    let base_dir = if let Some(home) = dirs::home_dir() {
        home.join(".hivelaunch")
    } else {
        std::env::temp_dir().join(".hivelaunch")
    };

    if let Err(err) = fs::create_dir_all(&base_dir) {
        log::warn!(
            "[DB] Failed to create base directory {:?}: {}",
            base_dir,
            err
        );
    }

    base_dir.join("hivelaunch.db")
}
