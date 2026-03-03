use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Show remote URLs.
pub fn do_remote(_project: &Path) -> Result<bool> {
    check_call("git", &["remote", "-v"])?;
    Ok(true)
}
