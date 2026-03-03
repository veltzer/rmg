use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Run git garbage collection.
pub fn do_gc(_project: &Path) -> Result<bool> {
    check_call("git", &["gc"])?;
    Ok(true)
}
