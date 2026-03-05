use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::config::AppConfig;

/// Discover git repositories based on the configuration.
/// Returns a sorted list of absolute paths to directories containing a `.git` folder.
pub fn discover_projects(config: &AppConfig) -> Result<Vec<PathBuf>> {
    let mut projects: Vec<PathBuf> = if !config.folders.is_empty() {
        // Explicit folder list
        config
            .folders
            .iter()
            .map(PathBuf::from)
            .filter(|p| p.join(".git").is_dir())
            .collect()
    } else if config.no_glob {
        // Current directory only – check immediate subdirectories
        std::fs::read_dir(".")
            .context("failed to read current directory")?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir() && p.join(".git").is_dir())
            .collect()
    } else {
        // Glob-based discovery: try the configured pattern, and if that
        // yields nothing also try "*" so rsmultigit works when immediate
        // subdirectories are already git repos.
        let mut found: Vec<PathBuf> = glob::glob(&config.glob)
            .context("invalid glob pattern")?
            .filter_map(|e| e.ok())
            .filter(|p| p.is_dir() && p.join(".git").is_dir())
            .collect();
        if found.is_empty() && config.glob == "*/*" {
            found = glob::glob("*")
                .context("invalid glob pattern")?
                .filter_map(|e| e.ok())
                .filter(|p| p.is_dir() && p.join(".git").is_dir())
                .collect();
        }
        found
    };

    // If no projects found and the current directory is itself a git repo, use it.
    if projects.is_empty() && PathBuf::from(".git").is_dir() {
        projects.push(PathBuf::from("."));
    }

    if !config.no_sort {
        projects.sort();
    }

    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::TempDir;

    fn make_config(overrides: impl FnOnce(&mut AppConfig)) -> AppConfig {
        let mut config = AppConfig::default();
        overrides(&mut config);
        config
    }

    #[test]
    fn discover_with_explicit_folders() {
        let tmp = TempDir::new().unwrap();
        let repo_a = tmp.path().join("repo_a");
        let repo_b = tmp.path().join("repo_b");
        let not_repo = tmp.path().join("not_a_repo");
        fs::create_dir_all(repo_a.join(".git")).unwrap();
        fs::create_dir_all(repo_b.join(".git")).unwrap();
        fs::create_dir_all(&not_repo).unwrap();

        let config = make_config(|c| {
            c.folders = vec![
                repo_a.to_string_lossy().to_string(),
                repo_b.to_string_lossy().to_string(),
                not_repo.to_string_lossy().to_string(),
            ];
        });

        let projects = discover_projects(&config).unwrap();
        assert_eq!(projects.len(), 2);
        assert!(projects.contains(&repo_a));
        assert!(projects.contains(&repo_b));
    }

    #[test]
    fn discover_with_folders_sorted() {
        let tmp = TempDir::new().unwrap();
        let repo_z = tmp.path().join("z_repo");
        let repo_a = tmp.path().join("a_repo");
        fs::create_dir_all(repo_z.join(".git")).unwrap();
        fs::create_dir_all(repo_a.join(".git")).unwrap();

        let config = make_config(|c| {
            c.folders = vec![
                repo_z.to_string_lossy().to_string(),
                repo_a.to_string_lossy().to_string(),
            ];
        });

        let projects = discover_projects(&config).unwrap();
        assert_eq!(projects[0], repo_a);
        assert_eq!(projects[1], repo_z);
    }

    #[test]
    fn discover_with_folders_unsorted() {
        let tmp = TempDir::new().unwrap();
        let repo_z = tmp.path().join("z_repo");
        let repo_a = tmp.path().join("a_repo");
        fs::create_dir_all(repo_z.join(".git")).unwrap();
        fs::create_dir_all(repo_a.join(".git")).unwrap();

        let config = make_config(|c| {
            c.no_sort = true;
            c.folders = vec![
                repo_z.to_string_lossy().to_string(),
                repo_a.to_string_lossy().to_string(),
            ];
        });

        let projects = discover_projects(&config).unwrap();
        // Should preserve insertion order (z first)
        assert_eq!(projects[0], repo_z);
        assert_eq!(projects[1], repo_a);
    }

    #[test]
    #[serial]
    fn discover_no_glob_finds_immediate_subdirs() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("repo1/.git")).unwrap();
        fs::create_dir_all(tmp.path().join("repo2/.git")).unwrap();
        fs::create_dir_all(tmp.path().join("plain_dir")).unwrap();

        std::env::set_current_dir(tmp.path()).unwrap();

        let config = make_config(|c| {
            c.no_glob = true;
        });

        let projects = discover_projects(&config).unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    #[serial]
    fn discover_glob_pattern() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("org/repo1/.git")).unwrap();
        fs::create_dir_all(tmp.path().join("org/repo2/.git")).unwrap();
        fs::create_dir_all(tmp.path().join("org/not_a_repo")).unwrap();

        std::env::set_current_dir(tmp.path()).unwrap();

        let config = make_config(|c| {
            c.glob = "org/*".to_string();
        });

        let projects = discover_projects(&config).unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    #[serial]
    fn discover_empty_returns_empty() {
        let tmp = TempDir::new().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();

        let config = AppConfig::default();
        let projects = discover_projects(&config).unwrap();
        assert!(projects.is_empty());
    }
}
