use std::fs;
use crate::common::{run_rsmultigit, stdout_str, stderr_str, setup_git_repos};

#[test]
fn list_projects_finds_immediate_subdirs() {
    let tmp = setup_git_repos(&["alpha", "beta"]);
    let output = run_rsmultigit(tmp.path(), &["list-projects"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("alpha"), "should find alpha: {stdout}");
    assert!(stdout.contains("beta"), "should find beta: {stdout}");
}

#[test]
fn list_projects_finds_nested_subdirs() {
    let tmp = setup_git_repos(&[]);
    // Create org/repo1, org/repo2
    let org = tmp.path().join("org");
    fs::create_dir_all(&org).unwrap();
    crate::common::init_git_repo(&org.join("repo1"));
    crate::common::init_git_repo(&org.join("repo2"));

    let output = run_rsmultigit(tmp.path(), &["list-projects"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("repo1"), "should find repo1: {stdout}");
    assert!(stdout.contains("repo2"), "should find repo2: {stdout}");
}

#[test]
fn no_projects_prints_message() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = run_rsmultigit(tmp.path(), &["list-projects"]);
    assert!(output.status.success());
    let stderr = stderr_str(&output);
    assert!(stderr.contains("no projects found"), "expected 'no projects found': {stderr}");
}

#[test]
fn no_print_no_projects_suppresses_message() {
    let tmp = tempfile::TempDir::new().unwrap();
    let output = run_rsmultigit(tmp.path(), &["--no-print-no-projects", "list-projects"]);
    assert!(output.status.success());
    let stderr = stderr_str(&output);
    assert!(!stderr.contains("no projects found"), "should suppress message: {stderr}");
}

#[test]
fn glob_flag_overrides_default() {
    let tmp = setup_git_repos(&["visible", "hidden"]);
    let output = run_rsmultigit(tmp.path(), &["--glob", "vis*", "list-projects"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("visible"), "should find visible: {stdout}");
    assert!(!stdout.contains("hidden"), "should not find hidden: {stdout}");
}

#[test]
fn folders_flag_selects_specific() {
    let tmp = setup_git_repos(&["aaa", "bbb", "ccc"]);
    let aaa = tmp.path().join("aaa").to_string_lossy().to_string();
    let ccc = tmp.path().join("ccc").to_string_lossy().to_string();
    let folders_arg = format!("{aaa},{ccc}");
    let output = run_rsmultigit(tmp.path(), &["--folders", &folders_arg, "list-projects"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("aaa"), "should find aaa: {stdout}");
    assert!(stdout.contains("ccc"), "should find ccc: {stdout}");
    assert!(!stdout.contains("bbb"), "should not find bbb: {stdout}");
}

#[test]
fn skips_non_git_directories() {
    let tmp = setup_git_repos(&["real_repo"]);
    fs::create_dir_all(tmp.path().join("not_a_repo")).unwrap();
    let output = run_rsmultigit(tmp.path(), &["list-projects"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("real_repo"), "should find real_repo: {stdout}");
    assert!(!stdout.contains("not_a_repo"), "should skip non-repo: {stdout}");
}
