use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::context;
use crate::context::localnode_key_pair_file;
use crate::error::{Detail, Result};
use crate::security::{Algorithm, PublicKeyAlgorithm};
use crate::security::ed25519::Ed25519;

use super::*;

pub struct Init<'a> {
  pub dir: &'a Path,
  pub force: bool,
}

impl<'a> Init<'a> {
  /// 指定されたディレクトリに新しいノードコンテキストを作成します。
  pub fn init(&self) -> Result<()> {
    // 既存の構成を上書きしないようにディレクトリが存在しないことを確認
    if self.dir.exists() {
      if !self.force {
        return Err(Detail::FileOrDirectoryExists { location: abs_path(self.dir) });
      } else {
        log::warn!("Overwriting the existing directory: {}", abs_path(self.dir))
      }
    } else {
      std::fs::create_dir_all(self.dir)?;
    }

    // ノード鍵の作成
    let local_dir = self.dir.join(context::DIR_SECURITY);
    create_dirs_if_not_exists(local_dir.as_path())?;
    let key_pair = Ed25519::generate_key_pair();
    let key_pair_path = local_dir.join(localnode_key_pair_file(Ed25519::id()));
    let mut file = File::create(&key_pair_path)?;
    file.write_all(key_pair.to_bytes().as_slice())?;
    log::info!("A node key is generated: {}", key_pair_path.to_string_lossy());

    Ok(())
  }
}