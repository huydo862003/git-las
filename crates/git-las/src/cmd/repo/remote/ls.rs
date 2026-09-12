//! git las repo remote ls
//! List remotes configured for the current repo

use crate::gitlas;
use crate::logger;

pub fn run() -> anyhow::Result<()> {
  let workspace = gitlas::load_workspace()?;
  let config = workspace.config();
  let name = workspace.get_current_repo_name()?;

  let repo = config
    .repos
    .iter()
    .find(|repo| repo.name == name)
    .ok_or_else(|| anyhow::anyhow!("repo '{name}' is not tracked"))?;

  if let Some(primary) = &repo.primary {
    logger::print_info(&format!("primary: {}  {}", primary.name, primary.url));
  } else {
    logger::print_info("primary: (not set)");
  }

  if repo.push_remotes.is_empty() {
    logger::print_info("remotes: (none, falls back to all global remotes)");
  } else {
    for remote in &repo.push_remotes {
      logger::print_info(&format!("  {}  {}", remote.name, remote.url));
    }
  }

  Ok(())
}
