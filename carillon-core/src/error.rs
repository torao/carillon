use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Detail {
  // ファイルまたはディレクトリが存在しない
  #[error("the file or directory does not exist: {location}")]
  FileOrDirectoryNotExist { location: String },

  // ファイルまたはディレクトリはすでに存在する
  #[error("the file or directory already exists: {location}")]
  FileOrDirectoryExists { location: String },

  // I/O に関連する一般的なエラー
  #[error(transparent)]
  IO(#[from] std::io::Error),

  // 公開鍵暗号: 鍵の復元に失敗
  #[error("Cannot recover the key from the specified byte array: {message}")]
  CannotRestoreKey { message: String },

  // 公開鍵暗号: 互換性のない鍵の変換
  #[error("incompatible key conversions: {message}")]
  IncompatibleKeyConversions { message: String },

  // 公開鍵暗号: 署名に失敗
  #[error("failed to sign: {message}")]
  FailedToSign { message: String },

  // 設定内容が不正
  #[error("{location}({line}:{column})")]
  InvalidConfig { location: String, line: u64, column: u64 },
}

pub type Result<T> = std::result::Result<T, Detail>;

impl Detail {
  pub fn file_or_directory_not_exist<T>(location: &Path) -> Result<T> {
    let loc = location.to_string_lossy().to_string();
    Err(Detail::FileOrDirectoryNotExist { location: loc })
  }
}
