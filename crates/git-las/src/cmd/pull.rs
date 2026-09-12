//! git las pull
//! Pull all tracked repos from their primary remote, cloning missing repos

use std::path::Path;

use crate::git;
use crate::gitlas;
use crate::logger;

pub fn run() -> anyhow::Result<()> {
  let workspace = gitlas::load_workspace()?;
  let config = workspace.config();

  if config.repos.is_empty() {
    logger::print_info("no repos tracked");
    return Ok(());
  }

  let total = config.repos.len() as u64 + 1;
  let progress = logger::create_progress_bar(total);

  // Pull each tracked repo from its primary remote
  for repo in &config.repos {
    progress.set_message(repo.name.clone());
    pull_or_clone(&repo.name, &repo.primary, &repo.path)?;
    progress.inc(1);
  }

  // Pull the meta-repo itself
  progress.set_message("meta-repo");
  pull_from_primary(&config.meta.name, &config.meta.primary, &config.meta.path)?;
  progress.inc(1);
  progress.finish_and_clear();

  Ok(())
}

fn pull_or_clone(
  name: &str,
  primary: &Option<gitlas::ResolvedRemote>,
  path: &Path,
) -> anyhow::Result<()> {
  let Some(primary) = primary else {
    logger::print_skip(&format!("{name}: no primary remote set"));
    return Ok(());
  };

  if !path.exists() {
    logger::print_info(&format!("{name}: cloning from {}...", primary.name));
    git::clone_repo(&primary.url, path)?;
    return Ok(());
  }

  pull_from_remote(name, primary, path)
}

fn pull_from_primary(
  name: &str,
  primary: &Option<gitlas::ResolvedRemote>,
  path: &Path,
) -> anyhow::Result<()> {
  let Some(primary) = primary else {
    logger::print_skip(&format!("{name}: no primary remote set"));
    return Ok(());
  };
  pull_from_remote(name, primary, path)
}

fn pull_from_remote(
  name: &str,
  remote: &gitlas::ResolvedRemote,
  path: &Path,
) -> anyhow::Result<()> {
  git::ensure_remote(path, &remote.name, &remote.url)?;
  if git::pull_from(path, &remote.name)? {
    logger::print_ok(&format!("{name} -> {}: pull", remote.name));
  } else {
    logger::print_err(&format!("{name} -> {}: pull failed", remote.name));
  }
  Ok(())
}
