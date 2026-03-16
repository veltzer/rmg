use std::path::Path;

use anyhow::Result;

use crate::commands::count::{has_untracked, is_dirty};
use crate::subprocess_utils::check_call;

/// Commit all staged and unstaged changes with a message.
/// Skips repos that have no changes.
pub fn do_commit(project: &Path, message: &str) -> Result<bool> {
    if !is_dirty(project)? && !has_untracked(project)? {
        return Ok(false);
    }
    check_call("git", &["add", "-A"])?;
    check_call("git", &["commit", "-m", message])?;
    Ok(true)
}
