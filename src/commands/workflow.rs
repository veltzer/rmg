use std::path::Path;

use anyhow::Result;

/// Check if a GitHub Actions workflow file exists for repos that have a Makefile.
/// Returns Some(message) if a Makefile exists but no workflow file is found.
pub fn check_workflow_exists_for_makefile(_project: &Path) -> Result<Option<String>> {
    if !Path::new("Makefile").exists() {
        return Ok(None);
    }

    let workflow_dir = Path::new(".github/workflows");
    if !workflow_dir.is_dir() {
        return Ok(Some("Makefile exists but no .github/workflows directory".to_string()));
    }

    let has_workflow = std::fs::read_dir(workflow_dir)?
        .filter_map(|e| e.ok())
        .any(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".yml") || name.ends_with(".yaml")
        });

    if has_workflow {
        Ok(None)
    } else {
        Ok(Some(
            "Makefile exists but no workflow YAML files found".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn no_makefile_returns_none() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        let result = check_workflow_exists_for_makefile(tmp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn makefile_without_workflows_dir() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        fs::write(tmp.path().join("Makefile"), "all:\n\techo hi").unwrap();
        let result = check_workflow_exists_for_makefile(tmp.path()).unwrap();
        assert_eq!(
            result.unwrap(),
            "Makefile exists but no .github/workflows directory"
        );
    }

    #[test]
    #[serial]
    fn makefile_with_empty_workflows_dir() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        fs::write(tmp.path().join("Makefile"), "all:\n\techo hi").unwrap();
        fs::create_dir_all(tmp.path().join(".github/workflows")).unwrap();
        let result = check_workflow_exists_for_makefile(tmp.path()).unwrap();
        assert_eq!(
            result.unwrap(),
            "Makefile exists but no workflow YAML files found"
        );
    }

    #[test]
    #[serial]
    fn makefile_with_workflow_yml() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        fs::write(tmp.path().join("Makefile"), "all:\n\techo hi").unwrap();
        let wf_dir = tmp.path().join(".github/workflows");
        fs::create_dir_all(&wf_dir).unwrap();
        fs::write(wf_dir.join("ci.yml"), "name: CI").unwrap();
        let result = check_workflow_exists_for_makefile(tmp.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    #[serial]
    fn makefile_with_workflow_yaml() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        fs::write(tmp.path().join("Makefile"), "all:\n\techo hi").unwrap();
        let wf_dir = tmp.path().join(".github/workflows");
        fs::create_dir_all(&wf_dir).unwrap();
        fs::write(wf_dir.join("build.yaml"), "name: Build").unwrap();
        let result = check_workflow_exists_for_makefile(tmp.path()).unwrap();
        assert!(result.is_none());
    }
}
