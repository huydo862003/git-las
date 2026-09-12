//! git las repo remote rm
//! Remove a remote from the current repo

use crate::gitlas;
use crate::logger;

pub fn run(remote: String) -> anyhow::Result<()> {
  let mut workspace = gitlas::load_workspace()?;
  let name = workspace.get_current_repo_name()?;

  workspace.remove_repo_remote(&name, &remote)?;
  workspace.save()?;

  logger::print_ok(&format!("{name}: removed remote '{remote}'"));
  Ok(())
}
