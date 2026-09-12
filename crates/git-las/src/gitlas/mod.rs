//! The `.gitlas/` workspace

mod resolve;
mod types;
mod validate;

pub use types::resolved::*;

use types::raw::*;

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::git;
use crate::gitlas::resolve::resolve_config;
use crate::gitlas::validate::validate_config;

const GITLAS_DIR: &str = ".gitlas";
const LOCAL_DIR: &str = ".local";
const CONFIG_FILE: &str = "config.toml";
const SECRETS_FILE: &str = "secrets.toml";
const LOCAL_GITIGNORE: &str = "*\n!.gitignore\n";

pub struct Workspace {
  root: PathBuf,
  config: RawConfig,
}

impl Workspace {
  /// Resolve and return the full config
  pub fn config(&self) -> ResolvedConfig {
    resolve_config(&self.config, &self.root)
  }

  /// Return the workspace root path
  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Infer the repo name from the current working directory
  pub fn get_current_repo_name(&self) -> anyhow::Result<String> {
    let cwd = std::env::current_dir()?;
    cwd
      .file_name()
      .and_then(|file_name| file_name.to_str())
      .map(|str_name| str_name.to_string())
      .ok_or_else(|| anyhow::anyhow!("could not determine repo name from current directory"))
  }

  /// Add a global remote
  pub fn add_remote(&mut self, name: String, url: String, user: String) {
    self
      .config
      .remotes
      .insert(name, RawRemoteConfig { url, user });
  }

  /// Remove a global remote
  pub fn remove_remote(&mut self, name: &str) -> anyhow::Result<()> {
    let refs: Vec<&str> = self
      .config
      .repo
      .iter()
      .filter(|(_, repo)| {
        repo
          .primary
          .as_ref()
          .is_some_and(|primary| primary.name == name)
          || repo.remotes.iter().any(|remote| remote.name == name)
      })
      .map(|(repo_name, _)| repo_name.as_str())
      .collect();

    if !refs.is_empty() {
      anyhow::bail!(
        "remote '{name}' is still referenced by repos: {}",
        refs.join(", ")
      );
    }

    if self.config.remotes.remove(name).is_none() {
      anyhow::bail!("remote '{name}' not found");
    }
    Ok(())
  }

  /// Add a tracked repo (no-op if already tracked)
  pub fn add_repo(&mut self, name: String) {
    self.config.repo.entry(name).or_default();
  }

  /// Remove a tracked repo
  pub fn remove_repo(&mut self, name: &str) -> anyhow::Result<()> {
    if self.config.repo.remove(name).is_none() {
      anyhow::bail!("repo '{name}' is not tracked");
    }
    Ok(())
  }

  /// Add a push remote to a repo
  pub fn add_repo_remote(
    &mut self,
    repo_name: &str,
    remote_name: String,
    repo_override: Option<String>,
    user_override: Option<String>,
  ) -> anyhow::Result<()> {
    if !self.config.remotes.contains_key(&remote_name) {
      anyhow::bail!("remote '{remote_name}' not found - add it with `git las remote add`");
    }

    let repo = self
      .config
      .repo
      .get_mut(repo_name)
      .ok_or_else(|| anyhow::anyhow!("repo '{repo_name}' is not tracked"))?;

    repo.remotes.retain(|existing| existing.name != remote_name);
    repo.remotes.push(RawRemote {
      name: remote_name,
      repo: repo_override,
      user: user_override,
    });
    Ok(())
  }

  /// Remove a push remote from a repo
  pub fn remove_repo_remote(&mut self, repo_name: &str, remote_name: &str) -> anyhow::Result<()> {
    let repo = self
      .config
      .repo
      .get_mut(repo_name)
      .ok_or_else(|| anyhow::anyhow!("repo '{repo_name}' is not tracked"))?;

    let before = repo.remotes.len();
    repo.remotes.retain(|existing| existing.name != remote_name);

    if repo.remotes.len() == before {
      anyhow::bail!("remote '{remote_name}' not found on repo '{repo_name}'");
    }
    Ok(())
  }

  /// Set the primary remote for a repo
  pub fn set_repo_primary(
    &mut self,
    repo_name: &str,
    remote_name: String,
    repo_override: Option<String>,
    user_override: Option<String>,
  ) -> anyhow::Result<()> {
    if !self.config.remotes.contains_key(&remote_name) {
      anyhow::bail!("remote '{remote_name}' not found - add it with `git las remote add`");
    }

    let repo = self
      .config
      .repo
      .get_mut(repo_name)
      .ok_or_else(|| anyhow::anyhow!("repo '{repo_name}' is not tracked"))?;

    repo.primary = Some(RawRemote {
      name: remote_name,
      repo: repo_override,
      user: user_override,
    });
    Ok(())
  }

  /// Add a token for a global remote
  pub fn add_remote_token(&self, remote_name: &str, token: String) -> anyhow::Result<()> {
    let mut secrets: RawSecretsConfig = load_toml(get_secrets_path(&self.root))?;
    secrets
      .remotes
      .entry(remote_name.to_string())
      .or_default()
      .token = Some(token);
    save_toml(get_secrets_path(&self.root), &secrets)
  }

  /// Remove secrets for a global remote
  pub fn remove_remote_secrets(&self, remote_name: &str) -> anyhow::Result<()> {
    let mut secrets: RawSecretsConfig = load_toml(get_secrets_path(&self.root))?;
    secrets.remotes.remove(remote_name);
    save_toml(get_secrets_path(&self.root), &secrets)
  }

  /// Validate and save the config to disk
  pub fn save(&self) -> anyhow::Result<()> {
    validate_config(&self.config)?;
    save_toml(get_config_path(&self.root), &self.config)
  }
}

/// Parse, validate, resolve, and return a Workspace
pub fn load_workspace() -> anyhow::Result<Workspace> {
  let root = find_workspace_dir()?;
  let config: RawConfig = load_toml(get_config_path(&root))?;
  validate_config(&config)?;

  Ok(Workspace { root, config })
}

/// Locate the workspace root by walking up from cwd
pub fn find_workspace_dir() -> anyhow::Result<PathBuf> {
  if let Ok(dir) = std::env::var("GITLAS_DIR") {
    return Ok(PathBuf::from(dir));
  }
  let mut current = std::env::current_dir()?;
  loop {
    if current.join(GITLAS_DIR).is_dir() {
      return Ok(current);
    }
    match current.parent() {
      Some(parent) => current = parent.to_path_buf(),
      None => anyhow::bail!(
        "not inside a git-las workspace (no .gitlas/ found)\nhint: run `git las init` to create one"
      ),
    }
  }
}

/// Initialize the `.gitlas/` directory structure and git repo
pub fn init_dir(workspace: &Path, meta_repo: &str) -> anyhow::Result<()> {
  let gitlas_dir = get_gitlas_dir(workspace);
  let local_dir = gitlas_dir.join(LOCAL_DIR);
  fs::create_dir_all(&local_dir)?;

  write_if_absent(local_dir.join(".gitignore"), LOCAL_GITIGNORE)?;

  let config = RawConfig {
    meta: RawMeta {
      repo: Some(meta_repo.to_string()),
      ..Default::default()
    },
    ..Default::default()
  };
  write_if_absent(
    get_config_path(workspace),
    &toml::to_string_pretty(&config)?,
  )?;

  git::init_repo(&gitlas_dir)?;
  Ok(())
}

fn get_gitlas_dir(workspace: &Path) -> PathBuf {
  workspace.join(GITLAS_DIR)
}

fn get_config_path(workspace: &Path) -> PathBuf {
  get_gitlas_dir(workspace).join(CONFIG_FILE)
}

fn get_secrets_path(workspace: &Path) -> PathBuf {
  get_gitlas_dir(workspace).join(LOCAL_DIR).join(SECRETS_FILE)
}

fn write_if_absent(path: impl AsRef<Path>, content: &str) -> anyhow::Result<()> {
  if !path.as_ref().exists() {
    fs::write(path, content)?;
  }
  Ok(())
}

fn load_toml<T: Default + for<'de> Deserialize<'de>>(path: PathBuf) -> anyhow::Result<T> {
  if !path.exists() {
    return Ok(T::default());
  }
  let content = fs::read_to_string(&path)?;
  Ok(toml::from_str(&content)?)
}

fn save_toml<T: Serialize>(path: PathBuf, value: &T) -> anyhow::Result<()> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, toml::to_string_pretty(value)?)?;
  Ok(())
}
