use serde::Deserialize;
use std::fs::{create_dir_all, metadata, read_to_string};
use std::sync::OnceLock;

pub static SERVER_CONFIG: OnceLock<ServerConf> = OnceLock::<ServerConf>::new();

#[derive(Deserialize, Clone, Debug, Default)]
pub struct LogConfig {
    pub path: String,
    pub config_path: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ServerConf {
    pub log: LogConfig,
}

pub fn init_server_config(path: &String) -> &ServerConf {
    SERVER_CONFIG.get_or_init(|| {
        let source = read_to_string(path).expect("failed to read config file");
        toml::from_str(&source).unwrap()
    })
}

pub fn server_config() -> &'static ServerConf {
    if let Some(res) = SERVER_CONFIG.get() {
        res
    } else {
        panic!("server config uninitialized")
    }
}

pub fn init_server_log() {
    // 1. 获取配置信息
    let conf = server_config();

    // 2. 检查日志配置 .yaml 文件是否存在
    if metadata(&conf.log.config_path).is_err() {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.config_path
        );
    }

    // 3.尝试初始化日志存放目录
    match create_dir_all(&conf.log.path) {
        Ok(()) => {}
        Err(e) => {
            panic!(
                "{}:Failed to initialize log directory {}",
                e.to_string(),
                conf.log.path
            );
        }
    }

    // 4. 读取日志配置.yaml 文件的内容
    let content = match read_to_string(&conf.log.config_path) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    // 5. 替换日志文件的存放路径
    let config_content = content.replace("{$path}", &conf.log.path);
    println!("{}\n{}", "========log config========", config_content);

    // 6. 解析 yaml 格式的配置文件
    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Failed to parse the contents of the config file {} with error message :{}",
                conf.log.config_path,
                e.to_string()
            );
        }
    };

    // 7. 初始化日志配置
    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}
