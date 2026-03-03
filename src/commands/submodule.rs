use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Update submodules recursively.
pub fn submodule_update(_project: &Path) -> Result<bool> {
    check_call("git", &["submodule", "update", "--init", "--recursive"])?;
    Ok(true)
}
