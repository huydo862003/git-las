use std::{collections::HashMap, path::Path};

use crate::gitlas::{
  GITLAS_DIR, ResolvedConfig, ResolvedMeta, ResolvedRemote, ResolvedRemoteEntry, ResolvedRepo,
  types::raw::{RawConfig, RawRemote, RawRemoteConfig},
};

pub fn resolve_config(config: &RawConfig, workspace_root: &Path) -> ResolvedConfig {
  let meta_name = config.meta.repo.as_deref().unwrap_or("gitlas");

  let meta = ResolvedMeta {
    name: meta_name.to_string(),
    path: workspace_root.join(GITLAS_DIR),
    primary: resolve_primary(&config.remotes, &config.meta.primary, meta_name),
    push_remotes: resolve_push_remotes(&config.remotes, &config.meta.remotes, meta_name),
  };

  let repos = config
    .repo
    .iter()
    .map(|(name, repo)| ResolvedRepo {
      name: name.clone(),
      path: workspace_root.join(name),
      primary: resolve_primary(&config.remotes, &repo.primary, name),
      push_remotes: resolve_push_remotes(&config.remotes, &repo.remotes, name),
    })
    .collect();

  let remotes = config
    .remotes
    .iter()
    .map(|(name, remote)| ResolvedRemoteEntry {
      name: name.clone(),
      url: remote.url.clone(),
      user: remote.user.clone(),
    })
    .collect();

  ResolvedConfig {
    meta,
    repos,
    remotes,
  }
}

pub fn resolve_primary(
  global_remotes: &HashMap<String, RawRemoteConfig>,
  primary: &Option<RawRemote>,
  repo_name: &str,
) -> Option<ResolvedRemote> {
  primary.as_ref().map(|raw| {
    let global = &global_remotes[&raw.name];
    ResolvedRemote {
      name: raw.name.clone(),
      url: resolve_url(global, raw, repo_name),
    }
  })
}

pub fn resolve_push_remotes(
  global_remotes: &HashMap<String, RawRemoteConfig>,
  remotes: &[RawRemote],
  repo_name: &str,
) -> Vec<ResolvedRemote> {
  if !remotes.is_empty() {
    return remotes
      .iter()
      .map(|raw| {
        let global = &global_remotes[&raw.name];
        ResolvedRemote {
          name: raw.name.clone(),
          url: resolve_url(global, raw, repo_name),
        }
      })
      .collect();
  }
  // Fall back to all global remotes
  global_remotes
    .iter()
    .map(|(name, global)| ResolvedRemote {
      name: name.clone(),
      url: format!("{}/{}/{}.git", global.url, global.user, repo_name),
    })
    .collect()
}

pub fn resolve_url(global: &RawRemoteConfig, raw: &RawRemote, default_repo_name: &str) -> String {
  let user = raw.user.as_deref().unwrap_or(&global.user);
  let repo = raw.repo.as_deref().unwrap_or(default_repo_name);
  format!("{}/{}/{}.git", global.url, user, repo)
}
