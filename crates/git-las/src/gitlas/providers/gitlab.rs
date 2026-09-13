//! GitLab provider reads tokens from the glab CLI config

use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use super::{home_dir, GitProvider};

pub struct GitLab {
  pub host: String,
}

impl GitProvider for GitLab {
  fn read_cli_token(&self) -> Option<String> {
    let path = home_dir()?.join(".config/glab-cli/config.yml");
    let content = fs::read_to_string(path).ok()?;
    let config: GlabConfig = serde_yaml::from_str(&content).ok()?;
    config.hosts?.get(&self.host)?.token.clone()
  }
}

// ~/.config/glab-cli/config.yml: { hosts: { "gitlab.com": { token: "glpat-..." } } }
#[derive(Deserialize)]
struct GlabConfig {
  hosts: Option<HashMap<String, GlabHost>>,
}

#[derive(Deserialize)]
struct GlabHost {
  token: Option<String>,
}
