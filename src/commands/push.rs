use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Push the current branch to origin.
pub fn do_push(_project: &Path) -> Result<bool> {
    check_call("git", &["push"])?;
    Ok(true)
}
