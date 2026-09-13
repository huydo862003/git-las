//! GitHub provider reads tokens from the gh CLI config

use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use super::{home_dir, GitProvider};

pub struct GitHub {
  pub host: String,
}

impl GitProvider for GitHub {
  fn read_cli_token(&self) -> Option<String> {
    let path = home_dir()?.join(".config/gh/hosts.yml");
    let content = fs::read_to_string(path).ok()?;
    let hosts: HashMap<String, GhHost> = serde_yaml::from_str(&content).ok()?;
    hosts.get(&self.host)?.oauth_token.clone()
  }
}

// ~/.config/gh/hosts.yml: { "github.com": { oauth_token: "gho_..." } }
#[derive(Deserialize)]
struct GhHost {
  oauth_token: Option<String>,
}
