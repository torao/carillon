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
      Ok(())
    })
  }
}

fn bootstrap<PROCESS>(process: PROCESS) -> Result<()> where PROCESS: Fn() -> Result<()> {
  // TODO シグナルをハンドルして終了するか繰り返すかを判断する
  process()
}
