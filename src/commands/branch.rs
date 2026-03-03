use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::{capture_output, check_call};

/// Show local branches.
pub fn branch_local(_project: &Path) -> Result<()> {
    check_call("git", &["branch"])
}

/// Show remote branches.
pub fn branch_remote(_project: &Path) -> Result<()> {
    check_call("git", &["branch", "-r"])
}

/// Show the GitHub default branch (via `gh repo view`).
pub fn branch_github(_project: &Path) -> Result<()> {
    let output = capture_output(
        "gh",
        &["repo", "view", "--json", "defaultBranchRef", "-q", ".defaultBranchRef.name"],
    )?;
    println!("{output}");
    Ok(())
}
