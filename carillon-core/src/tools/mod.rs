use std::path::Path;

use crate::Result;

pub mod init;

pub fn create_dirs_if_not_exists(dir: &Path) -> Result<()> {
  if !dir.is_dir() {
    std::fs::create_dir_all(dir)?;
  }
  Ok(())
}

pub fn abs_path<P: AsRef<Path>>(path: P) -> String {
  let path = path.as_ref();
  let path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir().unwrap().join(path)
  };
  path.display().to_string()
}
