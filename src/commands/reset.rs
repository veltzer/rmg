use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Reset HEAD with --hard.
pub fn reset_hard(_project: &Path) -> Result<bool> {
    check_call("git", &["reset", "--hard", "HEAD"])?;
    Ok(true)
}

/// Reset HEAD with --soft.
pub fn reset_soft(_project: &Path) -> Result<bool> {
    check_call("git", &["reset", "--soft", "HEAD"])?;
    Ok(true)
}

/// Reset HEAD with --mixed (default).
pub fn reset_mixed(_project: &Path) -> Result<bool> {
    check_call("git", &["reset", "--mixed", "HEAD"])?;
    Ok(true)
}
