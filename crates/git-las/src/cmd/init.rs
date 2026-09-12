//! git las init
//! Create a new workspace in the current directory

use std::io::{self, Write};

use crate::gitlas;

pub fn run() -> anyhow::Result<()> {
  let cwd = std::env::current_dir()?;

  if cwd.join(".gitlas").exists() {
    anyhow::bail!("workspace already initialized at {}", cwd.display());
  }

  let meta_repo = prompt_meta_repo_name()?;
  gitlas::init_dir(&cwd, &meta_repo)?;

  println!("initialized git-las workspace at {}", cwd.display());
  println!("meta-repo name: {meta_repo}");
  Ok(())
}

fn prompt_meta_repo_name() -> anyhow::Result<String> {
  print!("meta-repo name [gitlas]: ");
  io::stdout().flush()?;
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let name = input.trim().to_string();
  Ok(if name.is_empty() {
    "gitlas".to_string()
  } else {
    name
  })
}
