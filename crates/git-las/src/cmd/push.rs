//! git las push
//! Push all tracked repos and the meta-repo to their remotes

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

  // Push each tracked repo to its remotes
  for repo in &config.repos {
    progress.set_message(repo.name.clone());

    if !repo.path.exists() {
      logger::print_skip(&format!("{}: not cloned locally", repo.name));
      progress.inc(1);
      continue;
    }

    push_remotes(&repo.name, &repo.push_remotes, &repo.path)?;
    progress.inc(1);
  }

  // Push the meta-repo itself
  progress.set_message("meta-repo");
  push_remotes(
    &config.meta.name,
    &config.meta.push_remotes,
    &config.meta.path,
  )?;
  progress.inc(1);
  progress.finish_and_clear();

  Ok(())
}

fn push_remotes(name: &str, remotes: &[gitlas::ResolvedRemote], path: &Path) -> anyhow::Result<()> {
  for remote in remotes {
    git::ensure_remote(path, &remote.name, &remote.url)?;

    if git::push_all(path, &remote.name)? {
      logger::print_ok(&format!("{name} -> {}: push", remote.name));
    } else {
      logger::print_err(&format!("{name} -> {}: push failed", remote.name));
    }
  }
  Ok(())
}
