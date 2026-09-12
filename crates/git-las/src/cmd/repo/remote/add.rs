//! git las repo remote add
//! Add a remote to the current repo, optionally setting it as primary

use crate::gitlas;
use crate::logger;

pub fn run(
  remote: String,
  repo_name_override: Option<String>,
  user_override: Option<String>,
  primary: bool,
) -> anyhow::Result<()> {
  let mut workspace = gitlas::load_workspace()?;
  let name = workspace.get_current_repo_name()?;

  let message = if primary {
    workspace.set_repo_primary(&name, remote.clone(), repo_name_override, user_override)?;
    format!("{name}: set primary to '{remote}'")
  } else {
    workspace.add_repo_remote(&name, remote.clone(), repo_name_override, user_override)?;
    format!("{name}: added remote '{remote}'")
  };

  workspace.save()?;
  logger::print_ok(&message);
  Ok(())
}
