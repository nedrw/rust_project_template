# {{project-name}}

{{project_description}}

---

## 项目结构

```
src/
├── main.rs              # 主入口
├── lib.rs               # 库入口
├── error.rs             # 错误类型
├── prelude.rs           # 常用导出
└── optional/            # 可选功能模块
    ├── cli.rs           # 命令行参数解析
    ├── config.rs        # 配置文件管理
    └── log.rs           # 日志系统
```

---

## 快速开始

```bash
cargo build
cargo run -- --help
```

---

## 模板使用

```bash
cargo install cargo-generate
cargo generate --git https://github.com/{{author_name}}/{{project-name}}.git
```

---

## 许可证

MIT