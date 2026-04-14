use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Stash working-tree changes.
pub fn stash_push(project: &Path) -> Result<bool> {
    check_call(project, "git", &["stash", "push"])?;
    Ok(true)
}

/// Pop the most recent stash.
pub fn stash_pop(project: &Path) -> Result<bool> {
    check_call(project, "git", &["stash", "pop"])?;
    Ok(true)
}
