use std::fs;
use crate::common::{run_rmg, stdout_str, setup_git_repos};

#[test]
fn reports_missing_workflow_for_makefile() {
    let tmp = setup_git_repos(&["repo"]);
    fs::write(tmp.path().join("repo/Makefile"), "all:\n\techo hi\n").unwrap();

    let output = run_rmg(tmp.path(), &["check-workflow-exists-for-makefile"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.contains("repo"), "should report repo: {stdout}");
    assert!(stdout.contains("no .github/workflows directory"), "should report missing workflows: {stdout}");
}

#[test]
fn silent_when_no_makefile() {
    let tmp = setup_git_repos(&["repo"]);

    let output = run_rmg(tmp.path(), &["check-workflow-exists-for-makefile"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.is_empty(), "no Makefile means no output: {stdout}");
}

#[test]
fn silent_when_workflow_exists() {
    let tmp = setup_git_repos(&["repo"]);
    fs::write(tmp.path().join("repo/Makefile"), "all:\n\techo hi\n").unwrap();
    let wf_dir = tmp.path().join("repo/.github/workflows");
    fs::create_dir_all(&wf_dir).unwrap();
    fs::write(wf_dir.join("ci.yml"), "name: CI\n").unwrap();

    let output = run_rmg(tmp.path(), &["check-workflow-exists-for-makefile"]);
    assert!(output.status.success());
    let stdout = stdout_str(&output);
    assert!(stdout.is_empty(), "workflow exists, no output expected: {stdout}");
}
