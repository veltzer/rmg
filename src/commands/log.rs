use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Show recent commits (git log --oneline -n N).
pub fn do_log(_project: &Path, count: u32) -> Result<bool> {
    let n = count.to_string();
    check_call("git", &["log", "--oneline", "-n", &n])?;
    Ok(true)
}
