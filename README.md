# {{project-name}}

{{project_description}}

---

## 项目结构

```
src/
├── main.rs              # 主入口 (根据功能选择生成不同启动流程)
├── lib.rs               # 库入口 (共享类型与工具)
├── settings.rs          # [可选] 自文档化配置 (foundations settings)
├── telemetry.rs         # [可选] 遥测初始化 (日志/追踪/指标)
└── metrics.rs           # [可选] 自定义指标定义
```

---

## 功能特性 (初始化时选择)

| 功能 | 说明 | foundations feature |
|------|------|---------------------|
| 日志 (Logging) | 结构化日志，附带遥测服务端与指标 | `telemetry` |
| 分布式追踪 (Tracing) | Jaeger/OTLP 分布式链路追踪 | `telemetry` |
| jemalloc | 高性能内存分配器，适合长期运行服务 | `jemalloc` |
| 自文档配置 (Settings) | 带文档的 YAML 配置，自动生成默认配置 | `settings` |
| CLI 模块 | 命令行参数解析、配置加载、帮助生成 | `cli` (含 settings) |

---

## 快速开始

```bash
# 安装 cargo-generate
cargo install cargo-generate

# 从模板生成项目 (交互式选择功能)
cargo generate --git https://github.com/{{author_name}}/{{project-name}}.git
```

---

## 启动流程

选择不同功能后，生成的 `main.rs` 会有不同的启动流程：

### 完整功能 (日志 + 追踪 + CLI + jemalloc)

```
service_info!() → Cli 解析参数/加载配置 → telemetry::init() → 业务逻辑 → tele_driver.await
```

### 仅配置 (Settings / CLI)

```
service_info!() → 加载配置 → 业务逻辑
```

### 无功能 (与 cargo init 一致)

```
fn main() { println!("Hello, world!"); }
```

---

## 遥测端点

当启用日志或分布式追踪时，遥测服务端默认在随机端口启动，提供：

- `http://<addr>/metrics` — Prometheus 指标
- `http://<addr>/health` — 健康检查

生成默认配置文件查看所有可配置项：

```bash
cargo run -- -g config.yaml
```

---

## 许可证

MIT
