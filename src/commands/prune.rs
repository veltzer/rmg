use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Prune stale remote-tracking branches.
pub fn do_prune(_project: &Path) -> Result<bool> {
    check_call("git", &["remote", "prune", "origin"])?;
    Ok(true)
}
