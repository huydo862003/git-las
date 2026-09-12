//! Resolved config types consumed by commands

use std::path::PathBuf;

pub struct ResolvedConfig {
  pub meta: ResolvedMeta,
  pub repos: Vec<ResolvedRepo>,
  pub remotes: Vec<ResolvedRemoteEntry>,
}

pub struct ResolvedMeta {
  pub name: String,
  pub path: PathBuf,
  pub primary: Option<ResolvedRemote>,
  pub push_remotes: Vec<ResolvedRemote>,
}

pub struct ResolvedRepo {
  pub name: String,
  pub path: PathBuf,
  pub primary: Option<ResolvedRemote>,
  pub push_remotes: Vec<ResolvedRemote>,
}

pub struct ResolvedRemote {
  pub name: String,
  pub url: String,
}

pub struct ResolvedRemoteEntry {
  pub name: String,
  pub url: String,
  pub user: String,
}
