use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Commit all staged and unstaged changes with a message.
pub fn do_commit(_project: &Path, message: &str) -> Result<bool> {
    check_call("git", &["add", "-A"])?;
    check_call("git", &["commit", "-m", message])?;
    Ok(true)
}
