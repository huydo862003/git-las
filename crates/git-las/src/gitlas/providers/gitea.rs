//! Gitea/Forgejo provider reads tokens from the tea CLI config

use std::fs;

use serde::Deserialize;

use super::{home_dir, GitProvider};

pub struct Gitea {
  pub base_url: String,
}

impl GitProvider for Gitea {
  fn read_cli_token(&self) -> Option<String> {
    let path = home_dir()?.join(".tea/config.yml");
    let content = fs::read_to_string(path).ok()?;
    let config: TeaConfig = serde_yaml::from_str(&content).ok()?;
    let target = self.base_url.trim_end_matches('/');
    config
      .logins?
      .into_iter()
      .find(|login| login.url.trim_end_matches('/') == target)?
      .token
  }
}

// ~/.tea/config.yml: { logins: [{ url: "https://...", token: "..." }] }
#[derive(Deserialize)]
struct TeaConfig {
  logins: Option<Vec<TeaLogin>>,
}

#[derive(Deserialize)]
struct TeaLogin {
  url: String,
  token: Option<String>,
}
