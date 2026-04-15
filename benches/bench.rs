{% if include_benchmarks %}
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 测试配置加载性能
fn bench_config_loading(c: &mut Criterion) {
    {% if use_config %}
    c.bench_function("config_load", |b| {
        b.iter(|| {
            {{crate_name}}::config::init_config("config/base.{{config_format}}")
        })
    });
    {% endif %}
}

// 测试日志初始化性能
fn bench_logger_init(c: &mut Criterion) {
    {% if use_logging %}
    c.bench_function("logger_init", |b| {
        b.iter(|| {
            {{crate_name}}::log::init_simple_logger()
        })
    });
    {% endif %}
}

// 测试错误处理性能
fn bench_error_handling(c: &mut Criterion) {
    c.bench_function("error_creation", |b| {
        b.iter(|| {
            {{crate_name}}::error::Error::Generic("test error".to_string())
        })
    });

    c.bench_function("error_conversion", |b| {
        b.iter(|| {
            let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
            {{crate_name}}::error::Error::from(io_err)
        })
    });
}

{% if use_cli %}
// 测试 CLI 参数解析性能
fn bench_cli_parsing(c: &mut Criterion) {
    c.bench_function("cli_parse_default", |b| {
        b.iter(|| {
            {{crate_name}}::cmd::command::CliArgs::try_parse_from(["{{project-name}}"])
        })
    });

    c.bench_function("cli_parse_with_args", |b| {
        b.iter(|| {
            {{crate_name}}::cmd::command::CliArgs::try_parse_from([
                "{{project-name}}",
                "--config",
                "test.{{config_format}}",
                "--log-level",
                "debug"
            ])
        })
    });
}
{% endif %}

{% if use_logging %}
// 测试日志记录性能
fn bench_logging(c: &mut Criterion) {
    {{crate_name}}::log::init_simple_logger();

    c.bench_function("log_info", |b| {
        b.iter(|| {
            tracing::info!("Benchmark test message")
        })
    });

    c.bench_function("log_structured", |b| {
        b.iter(|| {
            tracing::info!(
                benchmark = true,
                iteration = black_box(42),
                "Structured benchmark log"
            )
        })
    });
}
{% endif %}

{% if use_config %}
// 测试配置解析性能
fn bench_config_parsing(c: &mut Criterion) {
    let config_content = r#"
app_name = "benchmark"

[log]
level = "info"
dir = "./logs"
prefix = "bench"
max_files = 10
console = true
"#;

    c.bench_function("config_parse", |b| {
        b.iter(|| {
            toml::from_str::<{{crate_name}}::config::AppConfig>(config_content)
        })
    });
}
{% endif %}

// 配置 criterion 测试组
criterion_group!(
    benches,
    bench_config_loading,
    bench_logger_init,
    bench_error_handling,
    {% if use_cli %}
    bench_cli_parsing,
    {% endif %}
    {% if use_logging %}
    bench_logging,
    {% endif %}
    {% if use_config %}
    bench_config_parsing,
    {% endif %}
);

criterion_main!(benches);

// 性能测试说明：
// 1. 使用 criterion 进行基准测试
// 2. 测试项目核心功能的性能
// 3. 结果会生成详细的性能报告
// 4. 可以与之前的基准进行对比
//
// 运行性能测试：
// cargo bench
//
// 查看详细报告：
// cargo bench -- --verbose
{% endif %}
