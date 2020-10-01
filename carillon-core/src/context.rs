use std::io;
use std::path::{Path, PathBuf};

use toml;

use crate::error::{Detail, Result};
use crate::security::KeyPair;

pub const DIR_SECURITY: &str = "security";

pub struct Context<'ctx> {
  dir: Box<&'ctx Path>,
}

impl<'ctx> Context<'ctx> {
  /// 指定されたローカルディレクトリにコンテキストをマップする。
  pub fn new(dir: &Path) -> Result<Box<Context>> {
    if !dir.exists() || !dir.is_dir() {
      Err(Detail::FileOrDirectoryNotExist { location: dir.to_string_lossy().to_string() })
    } else {
      Ok(Box::new(Context { dir: Box::new(dir.clone()) }))
    }

    let conf_file = dir.join("carillon.toml");
    let conf = std::fs::read_to_string(conf_file)?;
    let conf: Settings = toml::from_str(conf.as_str())?;
  }

  fn absolute_path(&self, path: &str) -> PathBuf {
    self.dir.join(path)
  }
}

pub fn localnode_key_pair_file(key_algorithm: &str) -> String {
  format!("localnode_{}", key_algorithm)
}

#[derive(Debug, Deserialize)]
struct Settings {
  node: Node
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