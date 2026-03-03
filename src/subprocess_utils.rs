use std::process::Command;

use anyhow::{bail, Result};

/// Run a command inside a Python virtualenv (.venv/bin/{cmd}).
/// Equivalent to pymultigit's check_call_ve.
pub fn check_call_ve(args: &[&str]) -> Result<()> {
    if args.is_empty() {
        bail!("check_call_ve requires at least one argument");
    }
    let venv_cmd = format!(".venv/bin/{}", args[0]);
    let status = Command::new(&venv_cmd)
        .args(&args[1..])
        .status()?;
    if !status.success() {
        bail!("{} failed with {}", venv_cmd, status);
    }
    Ok(())
}

/// Run a shell command and return its stdout as a String (trimmed).
pub fn capture_output(cmd: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(cmd).args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("{cmd} failed: {stderr}");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Run a shell command, inheriting stdout/stderr.
pub fn check_call(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        anyhow::bail!("{cmd} failed with {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_output_true() {
        let out = capture_output("echo", &["hello"]).unwrap();
        assert_eq!(out, "hello");
    }

    #[test]
    fn capture_output_trims_whitespace() {
        let out = capture_output("echo", &["  padded  "]).unwrap();
        assert_eq!(out, "padded");
    }

    #[test]
    fn capture_output_fails_on_bad_command() {
        let result = capture_output("false", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn check_call_succeeds() {
        assert!(check_call("true", &[]).is_ok());
    }

    #[test]
    fn check_call_fails() {
        assert!(check_call("false", &[]).is_err());
    }

    #[test]
    fn check_call_ve_empty_args() {
        assert!(check_call_ve(&[]).is_err());
    }
}
