use std::path::Path;

use crate::error::{Detail, Result};
use crate::context;

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
    let local_dir = self.dir.join(context::DIR_SECURITY).join(context::DIR_SECURITY_LOCAL);
    create_dirs_if_not_exists(local_dir.as_path())?;

    Ok(())
  }
}