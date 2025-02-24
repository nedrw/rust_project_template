use clap::Parser;

pub const BASE_CONFIG_PATH: &str = "config/base.toml";

#[derive(Debug, Parser)]
#[command(version, about = "about base server config")]
pub struct BaseConfigPath {
    /// the path of server config
    #[arg(short, long, default_value_t = String::from(BASE_CONFIG_PATH))]
    pub path: String,
}
