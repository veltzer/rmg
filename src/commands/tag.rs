use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// List local tags.
pub fn tag_local(_project: &Path) -> Result<bool> {
    check_call("git", &["tag"])?;
    Ok(true)
}

/// List remote tags.
pub fn tag_remote(_project: &Path) -> Result<bool> {
    check_call("git", &["ls-remote", "--tags", "origin"])?;
    Ok(true)
}
