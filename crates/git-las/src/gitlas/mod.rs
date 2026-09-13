//! The `.gitlas/` workspace

pub mod cred;
pub mod providers;
mod constants;
mod resolve;
mod types;
mod validate;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::git;
use crate::gitlas::constants::{CONFIG_FILE, GITLAS_DIR, GITLAS_LOCAL_PATH, LOCAL_GITIGNORE};
use crate::gitlas::resolve::resolve_config;
use crate::gitlas::types::{
  RawGitRemote, RawRemoteConfig, RawSecretsFile, RawWorkspaceConfig,
  RawWorkspaceMeta,
};
use crate::gitlas::validate::validate_config;
use crate::types::WorkspaceConfig;

pub struct Workspace {
  root: PathBuf,
  config: RawWorkspaceConfig,
}

impl Workspace {
  /// Find the workspace root, load and validate its config
  pub fn load() -> anyhow::Result<Workspace> {
    // Walk up from cwd to find the nearest `.gitlas/` directory
    let root = if let Ok(dir) = std::env::var("GITLAS_DIR") {
      // Env var overrides for testing
      PathBuf::from(dir)
    } else {
      let mut current = std::env::current_dir()?;
      loop {
        if current.join(GITLAS_DIR).is_dir() {
          break current;
        }
        match current.parent() {
          Some(parent) => current = parent.to_path_buf(),
          None => anyhow::bail!(
            r#"not inside a git-las workspace (no .gitlas/ found)
hint: run `git las init` to create one"#
          ),
        }
      }
    };

    let config_path = root.join(GITLAS_DIR).join(CONFIG_FILE);
    // Default to empty config if the file does not exist yet
    let config: RawWorkspaceConfig = if config_path.exists() {
      toml::from_str(&fs::read_to_string(&config_path)?)?
    } else {
      RawWorkspaceConfig::default()
    };

    validate_config(&config)?; // Callers can assume the config is valid

    Ok(Workspace { root, config })
  }

  /// Resolve raw config into fully-computed paths and URLs
  pub fn config(&self) -> WorkspaceConfig {
    resolve_config(&self.config, &self.root)
  }

  /// Return the workspace root path
  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Infer the current repo name from the working directory name
  pub fn get_current_repo_name(&self) -> anyhow::Result<String> {
    let cwd = std::env::current_dir()?;
    cwd
      .file_name()
      .and_then(|file_name| file_name.to_str())
      .map(|str_name| str_name.to_string())
      .ok_or_else(|| anyhow::anyhow!("could not determine repo name from current directory"))
  }

  /// Register a new global remote
  pub fn add_remote(&mut self, name: String, url: String, user: String) {
    self
      .config
      .remotes
      .insert(name, RawRemoteConfig { url, user });
  }

  /// Remove a global remote, refusing if any repo still references it
  pub fn remove_remote(&mut self, name: &str) -> anyhow::Result<()> {
    // Removal would silently break any repo that references this remote
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

  /// Track a repo in the workspace (no-op if already tracked)
  pub fn add_repo(&mut self, name: String) {
    self.config.repo.entry(name).or_default();
  }

  /// Stop tracking a repo
  pub fn remove_repo(&mut self, name: &str) -> anyhow::Result<()> {
    if self.config.repo.remove(name).is_none() {
      anyhow::bail!("repo '{name}' is not tracked");
    }
    Ok(())
  }

  /// Add (or replace) a push remote on a repo
  pub fn add_repo_remote(
    &mut self,
    repo_name: &str,
    remote_name: String,
    repo_override: Option<String>,
    user_override: Option<String>,
  ) -> anyhow::Result<()> {
    if !self.config.remotes.contains_key(&remote_name) {
      // Must exist to be resolvable
      anyhow::bail!("remote '{remote_name}' not found - add it with `git las remote add`");
    }

    let repo = self
      .config
      .repo
      .get_mut(repo_name)
      .ok_or_else(|| anyhow::anyhow!("repo '{repo_name}' is not tracked"))?;

    repo.remotes.retain(|existing| existing.name != remote_name); // Replace if already present
    repo.remotes.push(RawGitRemote {
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
      // No change means the remote was never listed
      anyhow::bail!("remote '{remote_name}' not found on repo '{repo_name}'");
    }
    Ok(())
  }

  /// Set which remote is used as the pull source for a repo
  pub fn set_repo_primary(
    &mut self,
    repo_name: &str,
    remote_name: String,
    repo_override: Option<String>,
    user_override: Option<String>,
  ) -> anyhow::Result<()> {
    if !self.config.remotes.contains_key(&remote_name) {
      // Must exist to be resolvable
      anyhow::bail!("remote '{remote_name}' not found - add it with `git las remote add`");
    }

    let repo = self
      .config
      .repo
      .get_mut(repo_name)
      .ok_or_else(|| anyhow::anyhow!("repo '{repo_name}' is not tracked"))?;

    repo.primary = Some(RawGitRemote {
      name: remote_name,
      repo: repo_override,
      user: user_override,
    });
    Ok(())
  }

  /// Write a token for a global remote to the secrets file
  pub fn write_token(&self, remote_name: &str, token: String) -> anyhow::Result<()> {
    let path = self.root.join(GITLAS_LOCAL_PATH).join("secrets.toml");
    let mut secrets: RawSecretsFile = if path.exists() {
      toml::from_str(&fs::read_to_string(&path)?)?
    } else {
      RawSecretsFile::default()
    };

    secrets.remotes.entry(remote_name.to_string()).or_default().token = Some(token);

    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }
    write_secrets_file(&path, &toml::to_string_pretty(&secrets)?)?;
    Ok(())
  }

  /// Create the `.gitlas/` directory structure and initialise it as a git repo
  pub fn init(workspace: &Path, meta_repo: &str) -> anyhow::Result<()> {
    let gitlas_dir = workspace.join(GITLAS_DIR);

    if gitlas_dir.exists() {
      anyhow::bail!("workspace already initialized at {}", workspace.display());
    }

    let local_dir = workspace.join(GITLAS_LOCAL_PATH); // Holds secrets, gitignored

    fs::create_dir_all(&local_dir)?;

    let gitignore = local_dir.join(".gitignore"); // Excludes everything except itself
    if !gitignore.exists() {
      fs::write(&gitignore, LOCAL_GITIGNORE)?;
    }

    let config_path = gitlas_dir.join(CONFIG_FILE);
    let config = RawWorkspaceConfig {
      meta: RawWorkspaceMeta {
        repo: Some(meta_repo.to_string()),
        ..Default::default()
      },
      ..Default::default()
    };
    fs::write(&config_path, toml::to_string_pretty(&config)?)?;

    // Turns `.gitlas/` into the versioned meta-repo
    git::init(&gitlas_dir)?;
    Ok(())
  }

  /// Validate and persist the in-memory config to disk
  pub fn save(&self) -> anyhow::Result<()> {
    validate_config(&self.config)?; // Never persist a broken config

    let path = self.root.join(GITLAS_DIR).join(CONFIG_FILE);
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }
    fs::write(path, toml::to_string_pretty(&self.config)?)?;
    Ok(())
  }
}

// Write secrets atomically: temp file with 0o600 then rename, so the content is never visible at the target path with wrong permissions
#[cfg(unix)]
fn write_secrets_file(path: &Path, content: &str) -> anyhow::Result<()> {
  use std::os::unix::fs::OpenOptionsExt;

  let tmp = path.with_extension("tmp");
  fs::OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .mode(0o600)
    .open(&tmp)?
    .write_all(content.as_bytes())?;
  fs::rename(&tmp, path)?;
  Ok(())
}

#[cfg(not(unix))]
fn write_secrets_file(path: &Path, content: &str) -> anyhow::Result<()> {
  fs::write(path, content)?;
  Ok(())
}
