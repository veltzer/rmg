use std::path::Path;

use anyhow::Result;

use crate::subprocess_utils::check_call;

/// Hard-clean the repository (git clean -ffxd).
pub fn clean_hard(_project: &Path) -> Result<bool> {
    check_call("git", &["clean", "-ffxd"])?;
    Ok(true)
}

/// Soft-clean the repository: remove untracked files only (git clean -fd).
pub fn clean_soft(_project: &Path) -> Result<bool> {
    check_call("git", &["clean", "-fd"])?;
    Ok(true)
}

/// Run `make clean`.
pub fn clean_make(_project: &Path) -> Result<bool> {
    check_call("make", &["clean"])?;
    Ok(true)
}

/// Discard unstaged working-tree changes (git checkout .).
pub fn clean_git(_project: &Path) -> Result<bool> {
    check_call("git", &["checkout", "."])?;
    Ok(true)
}

/// Run `cargo clean` if `Cargo.toml` exists, otherwise skip.
pub fn clean_cargo(project: &Path) -> Result<bool> {
    if !project.join("Cargo.toml").exists() {
        return Ok(false);
    }
    check_call("cargo", &["clean"])?;
    Ok(true)
}
