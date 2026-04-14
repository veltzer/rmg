use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::{capture_output, check_call};

/// List local tags.
pub fn tag_local(project: &Path) -> Result<bool> {
    check_call(project, "git", &["tag"])?;
    Ok(true)
}

/// List remote tags.
pub fn tag_remote(project: &Path) -> Result<bool> {
    check_call(project, "git", &["ls-remote", "--tags", "origin"])?;
    Ok(true)
}

/// Check if local tags exist.
pub fn tag_has_local(project: &Path) -> Result<bool> {
    let output = capture_output(project, "git", &["tag"])?;
    Ok(!output.is_empty())
}

/// Check if remote tags exist.
pub fn tag_has_remote(project: &Path) -> Result<bool> {
    let output = capture_output(project, "git", &["ls-remote", "--tags", "origin"])?;
    Ok(!output.is_empty())
}
