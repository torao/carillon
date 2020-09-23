extern crate carillon;

use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use log;
use log4rs;

use carillon::context;
use carillon::error::{Detail, Result};

/// # Carillon CLI
///
/// Carillon ノードの初期化、起動、停止、状態参照を行うための CLI です。
///
fn main() {
  let matches = App::new("Carillon CIL")
    .version("1.0")
    .author("TAKAMI Torao <koiroha@gmail.com>")
    .about("A self-sufficient commandline interface for Carillon runtime.")
    .arg(Arg::with_name("verbose")
      .short("v")
      .long("verbose")
      .multiple(true)
      .help("Sets the level of verbosity"))
    .subcommand(SubCommand::with_name("init")
      .about("Create and initialize a new context directory.")
      .version("1.0")
      .author("TAKAMI Torao")
      .arg(Arg::with_name("DIR")
        .help("context directory (error if it already exists)")
        .required(true))
      .arg(Arg::with_name("force")
        .short("f")
        .long("force")
        .help("Overwrite without error if the directory exists."))
      .arg(Arg::with_name("debug")
        .short("d")
        .help("print debug information verbosely")))
    .subcommand(SubCommand::with_name("start")
      .about("Starts a Carillon node based on the specified context directory.")
      .version("1.0")
      .author("TAKAMI Torao")
      .arg(Arg::with_name("DIR")
        .help("Specify the context directory of node to be launched. If omitted, the current directory is used.")
        .required(false)
        .index(1))
      .arg(Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("Print debug information verbosely")))
    .get_matches();

  // CLI のログ出力設定
  let logging_config = {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Root};
    let stdout = ConsoleAppender::builder().build();
    let verbose = matches.is_present("verbose");
    let level = if verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    log4rs::config::Config::builder()
      .appender(Appender::builder().build("stdout", Box::new(stdout)))
      .build(Root::builder().appender("stdout").build(level))
      .unwrap()
  };
  if let Err(err) = log4rs::init_config(logging_config) {
    error(&err)
  }

  // コンテキストディレクトリの新規作成
  if let Some(matches) = matches.subcommand_matches("init") {
    let dir = Path::new(matches.value_of("DIR").unwrap());
    let force = matches.is_present("force");
    match init(dir, force) {
      Ok(()) => {
        log::info!("SUCCESS: The context directory was created successfully: {}", abs_path(dir))
      }
      Err(err) => error(&err),
    }
  }
}

fn bootstrap(dir: &Path) {
  let context = context::Context::new(dir);
}

/// 指定されたディレクトリに新しいノードコンテキストを作成します。
fn init(dir: &Path, force: bool) -> Result<()> {
  // 既存の構成を上書きしないようにディレクトリが存在しないことを確認
  if dir.exists() {
    if !force {
      return Err(Detail::FileOrDirectoryExists { location: abs_path(dir) });
    } else {
      log::warn!("Overwriting the existing directory: {}", abs_path(dir))
    }
  } else {
    std::fs::create_dir_all(dir)?;
  }

  // ノード鍵の作成
  let local_dir = dir.join(context::DIR_SECURITY).join(context::DIR_SECURITY_LOCAL);
  create_dir_if_not_exists(local_dir.as_path())?;

  Ok(())
}

fn create_dir_if_not_exists(dir: &Path) -> Result<()> {
  if !dir.exists() {
    std::fs::create_dir_all(dir)?;
  }
  Ok(())
}

fn abs_path(path: &Path) -> String {
  let path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir().unwrap().join(path)
  };
  path.display().to_string()
}

fn error(err: &dyn std::error::Error) {
  eprintln!("ERROR: {}", err.to_string());
  exit(1)
}
