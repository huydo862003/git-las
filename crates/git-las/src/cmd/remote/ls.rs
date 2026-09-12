//! git las remote ls
//! List all configured remotes

use crate::gitlas;
use crate::logger;

pub fn run() -> anyhow::Result<()> {
  let workspace = gitlas::load_workspace()?;
  let config = workspace.config();

  if config.remotes.is_empty() {
    logger::print_info("no remotes configured");
    return Ok(());
  }

  for remote in &config.remotes {
    logger::print_info(&format!("{}  {}  {}", remote.name, remote.url, remote.user));
  }

  Ok(())
}
