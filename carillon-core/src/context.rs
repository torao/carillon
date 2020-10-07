use std::path::Path;

use serde::Deserialize;
use toml;

use crate::error::{Detail, Result};

pub const DIR_SECURITY: &str = "security";

pub struct Context<'ctx> {
  dir: Box<&'ctx Path>,
  settings: Box<Settings>,
}

impl<'ctx> Context<'ctx> {
  /// 指定されたローカルディレクトリにコンテキストをマップする。
  pub fn new(dir: &Path) -> Result<Box<Context>> {
    if !dir.exists() || !dir.is_dir() {
      return Err(Detail::FileOrDirectoryNotExist { location: dir.to_string_lossy().to_string() });
    }

    log::debug!("Loading the configuration: {}", dir.to_string_lossy());
    let conf_file = dir.join("carillon.toml");
    let conf = std::fs::read_to_string(conf_file)?;
    let conf: Settings = toml::from_str(conf.as_str()).map_err(|err| {
      let position = err.line_col();
      Detail::InvalidConfig {
        message: err.to_string(),
        location: dir.to_string_lossy().to_string(),
        line: position.map(|x| x.0 as u64).unwrap_or(0u64),
        column: position.map(|x| x.1 as u64).unwrap_or(0u64),
      }
    })?;
    Ok(Box::new(Context { dir: Box::new(dir.clone()), settings: Box::new(conf) }))
  }
}

pub fn localnode_key_pair_file(key_algorithm: &str) -> String {
  format!("localnode_{}", key_algorithm)
}

#[derive(Debug, Deserialize)]
struct Settings {
  node: Node,
}

#[derive(Debug, Deserialize)]
struct Node {
  identity: Identity,
}

#[derive(Debug, Deserialize)]
struct Identity {
  method: Option<String>,
  private_key: Option<PrivateKey>,
}

#[derive(Debug, Deserialize)]
struct PrivateKey {
  algorithm: Option<String>,
  location: Option<String>,
}
