use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::capture_output;

/// Show the age of the last commit as a human-readable relative date.
pub fn do_age(_project: &Path) -> Result<Option<String>> {
    let output = capture_output("git", &["log", "-1", "--format=%cr"])?;
    if output.is_empty() {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}
