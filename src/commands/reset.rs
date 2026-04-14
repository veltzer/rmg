use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Reset HEAD with --hard.
pub fn reset_hard(project: &Path) -> Result<bool> {
    check_call(project, "git", &["reset", "--hard", "HEAD"])?;
    Ok(true)
}

/// Reset HEAD with --soft.
pub fn reset_soft(project: &Path) -> Result<bool> {
    check_call(project, "git", &["reset", "--soft", "HEAD"])?;
    Ok(true)
}

/// Reset HEAD with --mixed (default).
pub fn reset_mixed(project: &Path) -> Result<bool> {
    check_call(project, "git", &["reset", "--mixed", "HEAD"])?;
    Ok(true)
}
