extern crate carillon;

use std::path::Path;
use std::process::exit;

use clap::{App, Arg, SubCommand};
use log;
use log4rs;

use carillon::tools;

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

  if let Some(matches) = matches.subcommand_matches("init") {
    // コンテキストディレクトリの新規作成
    let init = tools::init::Init {
      dir: Path::new(matches.value_of("DIR").unwrap()),
      force: matches.is_present("force"),
    };
    match init.init() {
      Ok(()) => log::info!(
        "SUCCESS: The context directory was created successfully: {}",
        tools::abs_path(init.dir)
      ),
      Err(err) => error(&err),
    }
  } else if let Some(matches) = matches.subcommand_matches("start") {
    // ノードの起動
    let dir = match matches.value_of("DIR") {
      Some(dir) => Path::new(dir).to_path_buf(),
      None => std::env::current_dir().unwrap_or(Path::new(".").to_path_buf()),
    };
    let start = tools::start::Start { dir: &dir };
    match start.start() {
      Ok(()) => log::info!("The carillon node has been deactivated"),
      Err(err) => error(&err),
    }
  }
}

fn error(err: &dyn std::error::Error) {
  eprintln!("ERROR: {}", err.to_string());
  exit(1)
}
