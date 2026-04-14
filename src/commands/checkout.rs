use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Checkout a branch.
pub fn do_checkout(project: &Path, branch: &str) -> Result<bool> {
    check_call(project, "git", &["checkout", branch])?;
    Ok(true)
}
