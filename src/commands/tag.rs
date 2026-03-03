use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// List tags.
pub fn do_tag(_project: &Path) -> Result<bool> {
    check_call("git", &["tag"])?;
    Ok(true)
}
