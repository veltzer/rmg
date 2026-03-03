use std::fs;
use crate::common::{run_rmg, stdout_str, setup_git_repos};

#[test]
fn count_dirty_clean_repos() {
    let tmp = setup_git_repos(&["a", "b"]);
    let output = run_rmg(tmp.path(), &["--stats", "count-dirty"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("0/2"), "clean repos should show 0/2: {stdout}");
}

#[test]
fn count_dirty_with_modified_file() {
    let tmp = setup_git_repos(&["clean", "dirty"]);
    // Create and commit a file in dirty, then modify it
    let dirty_path = tmp.path().join("dirty");
    let file = dirty_path.join("file.txt");
    fs::write(&file, "original").unwrap();
    std::process::Command::new("git")
        .args(["add", "file.txt"])
        .current_dir(&dirty_path)
        .status()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "add file"])
        .current_dir(&dirty_path)
        .status()
        .unwrap();
    fs::write(&file, "modified").unwrap();

    let output = run_rmg(tmp.path(), &["--stats", "count-dirty"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("1/2"), "one dirty repo should show 1/2: {stdout}");
}

#[test]
fn untracked_detects_new_files() {
    let tmp = setup_git_repos(&["clean", "has_new"]);
    fs::write(tmp.path().join("has_new/untracked.txt"), "data").unwrap();

    let output = run_rmg(tmp.path(), &["--stats", "untracked"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("1/2"), "one repo with untracked should show 1/2: {stdout}");
}

#[test]
fn count_dirty_terse_suppresses_names() {
    let tmp = setup_git_repos(&["a", "b"]);
    fs::write(tmp.path().join("a/untracked.txt"), "x").unwrap();

    // With --terse and --stats, should only print the count line
    let output = run_rmg(tmp.path(), &["--terse", "--stats", "untracked"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert_eq!(stdout, "1/2");
}

#[test]
fn print_not_inverts_selection() {
    let tmp = setup_git_repos(&["a", "b"]);
    fs::write(tmp.path().join("a/untracked.txt"), "x").unwrap();

    // --print-not should show "b" (the one WITHOUT untracked)
    let output = run_rmg(tmp.path(), &["--print-not", "untracked"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("b"), "should print the non-matching repo: {stdout}");
    assert!(!stdout.contains("/a\n"), "should not print the matching repo");
}
