use std::path::Path;

use serde::Deserialize;
use toml;

use crate::error::{Detail, Result};
use crate::security::{KeyPair, PublicKeyImpl};
use crate::security::ed25519;
use crate::tools::init::{DEFAULT_KEY_ALGORITHM, DEFAULT_KEY_LOCATION};

pub const DIR_SECURITY: &str = "security";
pub const DEFAULT_IDENTITY_METHOD: &str = "private_key";

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

  fn public_key_algorithm(&self) -> Result<Box<PublicKeyImpl>> {
    if let Some(identity) = &self.settings.node.identity.method {
      if identity != DEFAULT_IDENTITY_METHOD {
        return Err(Detail::UnsupportedSetting {
          location: self.dir.to_string_lossy().to_string(),
          item: "identity method",
          value: identity.to_string(),
        });
      }
    }
    let algorithm_name = (if let Some(pk) = self.settings.node.identity.private_key {
      pk.algorithm
    } else {

    }
      .map(|pk| pk.algorithm)
      .flatten()
      .map(|a| a.to_string())
      .unwrap_or(DEFAULT_KEY_ALGORITHM.to_string());
    let algorithm = match algorithm_name.as_str() {
      "ed25519" => ed25519::algorithm(),
      unsupported => {
        return Err(Detail::UnsupportedSetting {
          location: self.dir.to_string_lossy().to_string(),
          item: "public-key algorithm",
          value: unsupported.to_string(),
        });
      }
    };
    Ok(Box::new(algorithm))
  }

  fn key_pair(&self) -> Result<Box<dyn KeyPair>> {
    let algorithm = self.public_key_algorithm()?;
    let location = self.settings.node.identity.private_key.map(|s| s.location).flatten().unwrap_or(DEFAULT_KEY_LOCATION.to_string());
    let path = Path::new(location.as_str());
    let bytes = std::fs::read(path)?;
    algorithm.restore_key_pair(&bytes)
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

#[derive(Debug, Deserialize, Default)]
struct PrivateKey {
  algorithm: Option<String>,
  location: Option<String>,
}
