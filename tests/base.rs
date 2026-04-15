{% if use_config %}
use std::path::PathBuf;
use tempfile::TempDir;

// 测试配置系统
mod config_tests {
    use super::*;

    #[test]
    fn test_load_valid_config() {
        // 创建临时配置文件
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test.toml");

        let config_content = r#"
app_name = "test-app"

[log]
level = "debug"
dir = "./logs"
prefix = "test"
max_files = 5
console = true
json = false
"#;

        std::fs::write(&config_path, config_content).unwrap();

        // 测试加载配置
        let config = {{crate_name}}::config::load_config(&config_path);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.app_name, "test-app");
        assert_eq!(config.log.level, "debug");
    }

    #[test]
    fn test_load_nonexistent_config() {
        let config = {{crate_name}}::config::load_config("nonexistent.toml");
        assert!(config.is_err());
    }

    #[test]
    fn test_config_default_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("minimal.toml");

        // 最小配置文件
        let config_content = r#"
app_name = "minimal"
"#;

        std::fs::write(&config_path, config_content).unwrap();

        let config = {{crate_name}}::config::load_config(&config_path).unwrap();
        assert_eq!(config.app_name, "minimal");

        // 验证默认值
        {% if use_logging %}
        assert_eq!(config.log.level, "info");
        assert_eq!(config.log.dir, "./logs");
        {% endif %}
    }

    #[test]
    fn test_config_with_extra_fields() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("extended.toml");

        let config_content = r#"
app_name = "extended"

[log]
level = "info"

[custom]
feature_enabled = true
timeout_seconds = 30
"#;

        std::fs::write(&config_path, config_content).unwrap();

        let config = {{crate_name}}::config::load_config(&config_path).unwrap();
        assert!(config.extra.is_some());
    }
}
{% endif %}

{% if use_logging %}
// 测试日志系统
mod log_tests {
    #[test]
    fn test_log_config_default() {
        let config = {{crate_name}}::log::LogConfig::default();

        assert_eq!(config.level, "info");
        assert_eq!(config.dir, "./logs");
        assert_eq!(config.prefix, "{{project-name}}");
        assert_eq!(config.max_files, 10);
        assert!(config.console);
        assert!(!config.json);
    }

    #[test]
    fn test_simple_logger_init() {
        // 测试简单日志初始化不应该崩溃
        {{crate_name}}::log::init_simple_logger();

        // 使用日志宏
        tracing::info!("Test info message");
        tracing::debug!("Test debug message");
        tracing::warn!("Test warn message");
        tracing::error!("Test error message");

        // 如果能执行到这里，说明日志系统正常
        assert!(true);
    }

    #[test]
    fn test_logger_with_custom_config() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().to_str().unwrap();

        let config = {{crate_name}}::log::LogConfig {
            level: "debug".to_string(),
            dir: log_dir.to_string(),
            prefix: "test".to_string(),
            max_files: 3,
            console: false,
            json: false,
        };

        {{crate_name}}::log::init_logger(config);

        tracing::info!("Test message in custom log");

        // 验证日志目录是否存在
        assert!(std::path::Path::new(log_dir).exists());
    }
}
{% endif %}

// 测试错误处理
mod error_tests {
    #[test]
    fn test_error_generic() {
        let error = {{crate_name}}::error::Error::Generic("test error".to_string());
        assert_eq!(error.to_string(), "Generic test error");
    }

    #[test]
    fn test_error_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = {{crate_name}}::error::Error::IO(io_error);
        assert!(error.to_string().contains("file not found"));
    }

    #[test]
    fn test_result_type() {
        use {{crate_name}}::prelude::Result;

        let success: Result<i32> = Ok(42);
        assert!(success.is_ok());

        let failure: Result<i32> = Err({{crate_name}}::error::Error::Generic("failed".to_string()));
        assert!(failure.is_err());
    }
}

{% if use_cli %}
// 测试 CLI 命令解析
mod cli_tests {
    use clap::Parser;

    #[test]
    fn test_cli_default_args() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from(["{{project-name}}"]);
        assert!(args.is_ok());

        let args = args.unwrap();
        assert!(args.config.is_none());
        assert!(args.command.is_none());
    }

    #[test]
    fn test_cli_with_config() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "--config",
            "custom.toml"
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.config, Some("custom.toml".to_string()));
    }

    #[test]
    fn test_cli_with_log_level() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "--log-level",
            "debug"
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert_eq!(args.log_level, Some("debug".to_string()));
    }

    #[test]
    fn test_cli_version_command() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "version"
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(matches!(args.command, Some({{crate_name}}::cmd::command::Commands::Version)));
    }

    #[test]
    fn test_cli_info_command() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "info"
        ]);

        assert!(args.is_ok());
        let args = args.unwrap();
        assert!(matches!(args.command, Some({{crate_name}}::cmd::command::Commands::Info)));
    }

    {% if use_logging %}
    #[test]
    fn test_cli_log_show_command() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "log",
            "show"
        ]);

        assert!(args.is_ok());
    }
    {% endif %}

    {% if use_config %}
    #[test]
    fn test_cli_config_show_command() {
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from([
            "{{project-name}}",
            "config",
            "show",
            "--format",
            "toml"
        ]);

        assert!(args.is_ok());
    }
    {% endif %}
}
{% endif %}

// 测试 prelude 模块
mod prelude_tests {
    #[test]
    fn test_prelude_exports() {
        // 验证 prelude 导出了必要的类型
        use {{crate_name}}::prelude::{Error, Result};

        let _: Result<i32> = Ok(42);
        let _: Error = Error::Generic("test".to_string());
    }

    #[test]
    fn test_wrapper_struct() {
        use {{crate_name}}::prelude::W;

        let wrapped = W(42);
        assert_eq!(wrapped.0, 42);
    }
}

{% if include_examples %}
// 示例集成测试
mod example_tests {
    #[test]
    fn test_example_integration() {
        // 这个测试展示了如何使用项目的各个模块

        {% if use_config %}
        // 加载配置
        {{crate_name}}::config::init_config("config/base.toml");
        let config = {{crate_name}}::config::config();
        assert!(!config.app_name.is_empty());
        {% endif %}

        {% if use_logging %}
        // 初始化日志
        {{crate_name}}::log::init_simple_logger();
        tracing::info!("Integration test running");
        {% endif %}

        // 使用错误处理
        use {{crate_name}}::prelude::Result;
        let result: Result<()> = Ok(());
        assert!(result.is_ok());

        {% if use_cli %}
        // 测试 CLI
        let args = {{crate_name}}::cmd::command::CliArgs::try_parse_from(["{{project-name}}"]);
        assert!(args.is_ok());
        {% endif %}
    }
}
{% endif %}

// 测试文件说明：
//
// 本测试文件包含以下测试模块：
//
// {% if use_config %}
// 1. config_tests: 测试配置系统的各种场景
//    - 加载有效配置
//    - 加载不存在配置的错误处理
//    - 默认值测试
//    - 扩展字段测试
// {% endif %}
//
// {% if use_logging %}
// 2. log_tests: 测试日志系统
//    - 默认配置测试
//    - 简单日志初始化
//    - 自定义配置初始化
// {% endif %}
//
// 3. error_tests: 测试错误处理系统
//    - Generic 错误
//    - IO 错误
//    - Result 类型
//
// {% if use_cli %}
// 4. cli_tests: 测试命令行参数解析
//    - 默认参数
//    - 配置文件参数
//    - 日志级别参数
//    - 各种子命令
// {% endif %}
//
// 5. prelude_tests: 测试 prelude 模块导出
//
// {% if include_examples %}
// 6. example_tests: 示例集成测试，展示如何使用各个模块
// {% endif %}
//
// 运行测试：
// ```bash
// # 运行所有测试
// cargo test
//
// # 运行特定测试模块
// cargo test config_tests
// cargo test log_tests
//
// # 显示测试输出
// cargo test -- --nocapture
//
// # 运行特定测试
// cargo test test_load_valid_config
// ```
