use std::path::{Path, PathBuf};

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
  }

  fn absolute_path(&self, path: &str) -> PathBuf {
    self.dir.join(path)
  }
}

pub fn localnode_key_pair_file(key_algorithm: &str) -> String {
  format!("localnode_{}", key_algorithm)
}