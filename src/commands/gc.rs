use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Run git garbage collection.
pub fn do_gc(project: &Path) -> Result<bool> {
    check_call(project, "git", &["gc"])?;
    Ok(true)
}
