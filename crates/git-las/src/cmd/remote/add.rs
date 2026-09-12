//! git las remote add
//! Register a new remote with a base URL and username

use crate::gitlas;
use crate::logger;

pub fn run(name: String, url: String, user: String, token: Option<String>) -> anyhow::Result<()> {
  let mut workspace = gitlas::load_workspace()?;

  workspace.add_remote(name.clone(), url, user);

  if let Some(token) = token {
    workspace.add_remote_token(&name, token)?;
  }

  workspace.save()?;
  logger::print_ok(&format!("added remote '{name}'"));
  Ok(())
}
