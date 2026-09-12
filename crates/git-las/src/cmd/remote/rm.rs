//! git las remote rm
//! Remove a remote if no tracked repos reference it

use crate::gitlas;
use crate::logger;

pub fn run(name: String) -> anyhow::Result<()> {
  let mut workspace = gitlas::load_workspace()?;

  workspace.remove_remote(&name)?;
  workspace.remove_remote_secrets(&name)?;
  workspace.save()?;

  logger::print_ok(&format!("removed remote '{name}'"));
  Ok(())
}
