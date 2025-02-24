use clap::Parser;
use crate::cmd::command::BaseConfigPath;
use crate::config::{init_server_config, init_server_log};

mod prelude;
mod error;
mod config;
mod cmd;

fn main() {
    let args = BaseConfigPath::parse();
    init_server_config(&args.path);
    init_server_log();
}
