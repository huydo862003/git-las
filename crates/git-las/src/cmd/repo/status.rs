//! git las repo status
//! Show clean/dirty status of all tracked repos

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

  for repo in &config.repos {
    if !repo.path.exists() {
      logger::print_skip(&format!("{}  not cloned locally", repo.name));
      continue;
    }

    let output = git::get_status_short(&repo.path)?;
    let label = if output.trim().is_empty() {
      "clean"
    } else {
      "dirty"
    };
    logger::print_info(&format!("{}  {label}", repo.name));
  }

  Ok(())
}
