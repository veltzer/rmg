use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::capture_output;

/// Returns `Some(output)` if `git status -s` produces any output (i.e., working tree is not clean).
pub fn do_status(project: &Path) -> Result<Option<String>> {
    let output = capture_output(project, "git", &["status", "-s"])?;
    if output.is_empty() {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}

/// Returns `Some(output)` if there are dirty (modified/staged) changes.
/// Uses `git diff --stat` to detect modifications.
pub fn do_dirty(project: &Path) -> Result<Option<String>> {
    let output = capture_output(project, "git", &["diff", "--stat"])?;
    if output.is_empty() {
        let staged = capture_output(project, "git", &["diff", "--cached", "--stat"])?;
        if staged.is_empty() {
            Ok(None)
        } else {
            Ok(Some(staged))
        }
    } else {
        Ok(Some(output))
    }
}
