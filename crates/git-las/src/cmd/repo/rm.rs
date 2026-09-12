//! git las repo rm
//! Stop tracking a repo

use crate::gitlas;
use crate::logger;

pub fn run() -> anyhow::Result<()> {
  let mut workspace = gitlas::load_workspace()?;
  let name = workspace.get_current_repo_name()?;

  workspace.remove_repo(&name)?;
  workspace.save()?;

  logger::print_ok(&format!("untracked '{name}'"));
  Ok(())
}
