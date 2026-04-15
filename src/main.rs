{% if use_cli %}
use clap::Parser;
use crate::optional::cli::CliArgs;
{% endif %}

{% if use_config %}
use crate::optional::config::{init_config, config};
{% endif %}

{% if use_logging %}
use crate::optional::log::{init_logger, LogConfig};
{% endif %}

mod prelude;
mod error;

{% if use_config or use_logging or use_cli %}
mod optional;
{% endif %}

fn main() {
    // 1. 解析命令行参数（如果启用）
    {% if use_cli %}
    let args = CliArgs::parse();
    let config_path = args.config.as_deref().unwrap_or("config/base.{{config_format}}");

    // 处理子命令
    if let Some(subcommand) = args.command {
        handle_subcommand(subcommand);
        return;
    }
    {% else %}
    let config_path = "config/base.{{config_format}}";
    {% endif %}

    {% if use_config %}
    // 2. 加载配置
    let cfg = init_config(config_path);

    {% if use_logging %}
    // 3. 初始化日志系统（保存 guard 保持日志线程活跃）
    let log_config = LogConfig {
        level: cfg.log.level.clone(),
        dir: cfg.log.dir.clone(),
        prefix: cfg.log.prefix.clone(),
        max_files: cfg.log.max_files,
        console: cfg.log.console,
        json: cfg.log.json,
    };
    let _log_guard = init_logger(log_config);
    {% endif %}

    {% else %}
    {% if use_logging %}
    // 2. 初始化日志系统（使用默认配置，保存 guard）
    let _log_guard = crate::optional::log::init_simple_logger();
    {% endif %}
    {% endif %}

    // 应用启动
    {% if use_logging %}
    tracing::info!("🚀 Application {{project-name}} starting...");
    tracing::info!("📦 Version: 0.1.0");
    tracing::info!("👤 Author: {{author_name}}");
    {% else %}
    println!("🚀 Application {{project-name}} starting...");
    println!("📦 Version: 0.1.0");
    println!("👤 Author: {{author_name}}");
    {% endif %}

    {% if use_config %}
    {% if use_logging %}
    tracing::info!("⚙️  Configuration loaded from: {}", config_path);
    tracing::info!("📱 App name: {}", cfg.app_name);

    if let Some(extra) = &cfg.extra {
        tracing::debug!("📋 Extra config: {:?}", extra);
    }
    {% else %}
    println!("⚙️  Configuration loaded from: {}", config_path);
    println!("📱 App name: {}", cfg.app_name);
    {% endif %}
    {% endif %}

    // 4. 运行应用主逻辑
    run();
}

{% if use_cli %}
fn handle_subcommand(subcommand: crate::optional::cli::Commands) {
    use crate::optional::cli::Commands;

    match subcommand {
        Commands::Version => {
            println!("{{project-name}} version 0.1.0");
        }
        Commands::Info => {
            println!("Application: {{project-name}}");
            println!("Description: {{project_description}}");
            println!("Author: {{author_name}} <{{author_email}}>");
        }
        Commands::Check { path } => {
            let config_path = path.unwrap_or_else(|| "config/base.{{config_format}}".to_string());
            println!("Checking configuration file: {}", config_path);
            // TODO: 实现配置检查逻辑
        }
        Commands::Example { name } => {
            let example_name = name.unwrap_or_default();
            println!("Running example: {}", example_name);
            // TODO: 实现示例运行逻辑
        }
    }
}
{% endif %}

fn run() {
    // TODO: 实现你的应用主逻辑

    {% if use_logging %}
    tracing::info!("✅ Application started successfully");
    tracing::info!("💡 Implement your main logic here");
    {% else %}
    println!("✅ Application started successfully");
    println!("💡 Implement your main logic here");
    {% endif %}

    // 示例：使用配置
    {% if use_config %}
    let cfg = config();

    {% if use_logging %}
    tracing::info!("📊 Application config:");
    tracing::info!("  - App name: {}", cfg.app_name);
    {% if use_logging %}
    tracing::info!("  - Log level: {}", cfg.log.level);
    tracing::info!("  - Log dir: {}", cfg.log.dir);
    {% endif %}
    {% else %}
    println!("📊 Application config:");
    println!("  - App name: {}", cfg.app_name);
    {% endif %}
    {% endif %}

    // 示例：错误处理
    use crate::prelude::*;
    let _result: Result<()> = Ok(());
    // let _result: Result<()> = Err(Error::Generic("Example error".to_string()));

    {% if use_logging %}
    tracing::info!("🎉 Application {{project-name}} is running!");
    {% else %}
    println!("🎉 Application {{project-name}} is running!");
    {% endif %}
}
