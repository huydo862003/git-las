//! Bitbucket provider reads tokens from the bb CLI config

use std::fs;

use serde::Deserialize;

use super::{home_dir, GitProvider};

pub struct Bitbucket;

impl GitProvider for Bitbucket {
  fn read_cli_token(&self) -> Option<String> {
    let path = home_dir()?.join(".config/bb/config.toml");
    let content = fs::read_to_string(path).ok()?;
    let config: BbConfig = toml::from_str(&content).ok()?;
    config.auth?.token
  }
}

// ~/.config/bb/config.toml: [auth] token = "..."
#[derive(Deserialize)]
struct BbConfig {
  auth: Option<BbAuth>,
}

#[derive(Deserialize)]
struct BbAuth {
  token: Option<String>,
}
