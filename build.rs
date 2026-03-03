use std::process::Command;

fn git(args: &[&str]) -> String {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn main() {
    let sha = git(&["rev-parse", "HEAD"]);
    let branch = git(&["rev-parse", "--abbrev-ref", "HEAD"]);
    let describe = git(&["describe", "--tags", "--always", "--dirty"]);

    let dirty = Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .status()
        .map(|s| if s.success() { "false" } else { "true" })
        .unwrap_or("unknown");

    let rustc_ver = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.split_whitespace().nth(1).map(|v| v.to_owned())
        })
        .unwrap_or_else(|| "unknown".to_owned());

    println!("cargo:rustc-env=RMG_GIT_SHA={sha}");
    println!("cargo:rustc-env=RMG_GIT_BRANCH={branch}");
    println!("cargo:rustc-env=RMG_GIT_DIRTY={dirty}");
    println!("cargo:rustc-env=RMG_GIT_DESCRIBE={describe}");
    println!("cargo:rustc-env=RMG_RUSTC_SEMVER={rustc_ver}");

    // Rebuild on git HEAD or branch ref changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    if let Ok(head) = std::fs::read_to_string(".git/HEAD") {
        if let Some(refpath) = head.trim().strip_prefix("ref: ") {
            println!("cargo:rerun-if-changed=.git/{refpath}");
        }
    }
}
