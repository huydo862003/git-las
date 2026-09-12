//! Raw config types for parsing/saving TOML

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawConfig {
  #[serde(default)]
  pub meta: RawMeta,
  #[serde(default)]
  pub remotes: HashMap<String, RawRemoteConfig>,
  #[serde(default)]
  pub repo: HashMap<String, RawRepo>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawMeta {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repo: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub primary: Option<RawRemote>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub remotes: Vec<RawRemote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawRemoteConfig {
  pub url: String,
  pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRemote {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repo: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawRepo {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub primary: Option<RawRemote>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub remotes: Vec<RawRemote>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawSecretsConfig {
  #[serde(default)]
  pub remotes: HashMap<String, RawRemoteSecrets>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RawRemoteSecrets {
  pub token: Option<String>,
}
