use std::path::{Path, PathBuf};

use serde::Deserialize;
use toml;

use crate::error::{Detail, Result};
use crate::security;
use crate::security::ed25519;

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
    let conf = std::fs::read_to_string(conf_file.as_path())?;
    let conf: Settings = toml::from_str(conf.as_str()).map_err(|err| {
      let position = err.line_col();
      Detail::InvalidConfig {
        message: err.to_string(),
        location: conf_file.to_string_lossy().to_string(),
        line: position.map(|x| x.0 as u64 + 1).unwrap_or(0u64),
        column: position.map(|x| x.1 as u64 + 1).unwrap_or(0u64),
      }
    })?;
    Ok(Box::new(Context { dir: Box::new(dir.clone()), settings: Box::new(conf) }))
  }

  pub fn key_pair(&self) -> Result<Box<dyn security::KeyPair>> {
    match &self.settings.node.identity {
      Identity::PrivateKey { algorithm, location } => {
        let algorithm = match algorithm.as_str() {
          "ed25519" => ed25519::algorithm(),
          unsupported => {
            return Err(Detail::UnsupportedSetting {
              location: self.dir.to_string_lossy().to_string(),
              item: "public-key algorithm",
              value: unsupported.to_string(),
            });
          }
        };
        let path = self.resolve(location.as_str());
        if !path.is_file() {
          Err(Detail::FileOrDirectoryNotExist {
            location: path.to_string_lossy().to_string()
          })
        } else {
          let bytes = std::fs::read(path)?;
          let key_pair = algorithm.restore_key_pair(&bytes)?;
          Ok(key_pair)
        }
      }
    }
  }

  /// このコンテキストのディレクトリを基準に指定されたパスを参照します。指定されたパスが絶対パスの場合は
  fn resolve(&self, path: &str) -> PathBuf {
    if Path::new(path).is_absolute() {
      PathBuf::from(path)
    } else {
      self.dir.join(path)
    }
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
enum Identity {
  #[serde(rename = "private_key")]
  PrivateKey {
    algorithm: String,
    location: String,
  },
}
