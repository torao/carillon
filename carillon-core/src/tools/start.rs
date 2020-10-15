use std::path::Path;

use crate::context;
use crate::Result;

pub struct Start<'a> {
  pub dir: &'a Path,
}

impl<'a> Start<'a> {
  pub fn start(&self) -> Result<()> {
    let dir = self.dir;
    bootstrap(move || {
      let context = context::Context::new(dir)?;
      let key_pair = context.key_pair()?;
      log::info!(
        "Booting Carillong node: at address {}",
        key_pair.public_key().address().to_string()
      );
      Ok(())
    })
  }
}

fn bootstrap<PROCESS>(process: PROCESS) -> Result<()>
where
  PROCESS: Fn() -> Result<()>,
{
  // TODO シグナルをハンドルして終了するか繰り返すかを判断する
  process()
}
