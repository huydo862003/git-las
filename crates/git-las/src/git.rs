//! Git operations used across commands

use std::path::Path;
use std::process::Command;

/// Run `git init` at the given path
pub fn init_repo(path: &Path) -> anyhow::Result<()> {
  let path_str = path.to_string_lossy();
  let status = Command::new("git").args(["init", &path_str]).status()?;
  if !status.success() {
    anyhow::bail!("git init failed at {}", path.display());
  }
  Ok(())
}

/// Clone a repo from `url` into `dest`
pub fn clone_repo(url: &str, dest: &Path) -> anyhow::Result<()> {
  let dest_str = dest.to_string_lossy();
  let status = Command::new("git")
    .args(["clone", url, &dest_str])
    .status()?;
  if !status.success() {
    anyhow::bail!("git clone failed for {url}");
  }
  Ok(())
}

/// Push all branches to a named remote
pub fn push_all(repo_path: &Path, remote: &str) -> anyhow::Result<bool> {
  let path_str = repo_path.to_string_lossy();
  Ok(
    Command::new("git")
      .args(["-C", &path_str, "push", remote, "--all"])
      .status()?
      .success(),
  )
}

/// Pull from a named remote
pub fn pull_from(repo_path: &Path, remote: &str) -> anyhow::Result<bool> {
  let path_str = repo_path.to_string_lossy();
  Ok(
    Command::new("git")
      .args(["-C", &path_str, "pull", remote])
      .status()?
      .success(),
  )
}

/// Get short status output for a repo
pub fn get_status_short(repo_path: &Path) -> anyhow::Result<String> {
  let path_str = repo_path.to_string_lossy();
  let output = Command::new("git")
    .args(["-C", &path_str, "status", "--short"])
    .output()?;
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Ensure a named remote exists with the given URL
pub fn ensure_remote(repo_path: &Path, name: &str, url: &str) -> anyhow::Result<()> {
  let path_str = repo_path.to_string_lossy();
  let output = Command::new("git")
    .args(["-C", &path_str, "remote"])
    .output()?;
  let remotes = String::from_utf8_lossy(&output.stdout);

  if remotes.lines().any(|line| line == name) {
    Command::new("git")
      .args(["-C", &path_str, "remote", "set-url", name, url])
      .status()?;
  } else {
    Command::new("git")
      .args(["-C", &path_str, "remote", "add", name, url])
      .status()?;
  }
  Ok(())
}

/// Check if a path is a git repository
pub fn is_git_repo(path: &Path) -> bool {
  path.join(".git").exists()
}
