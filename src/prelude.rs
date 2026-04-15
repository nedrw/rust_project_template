/// Prelude 模块 - 导出常用类型和函数
///
/// # 导出内容
///
/// - `Error`: 错误类型（来自 `error` 模块）
/// - `Result`: 结果类型（`Result<T, Error>`）
/// - `W`: 包装结构体（用于扩展类型功能）
///
/// {% if use_logging %}
/// - 日志宏: `info`, `debug`, `warn`, `error`, `trace` (来自 tracing)
/// {% endif %}
///
/// {% if use_config %}
/// - `AppConfig`: 应用配置类型（来自 `config` 模块）
/// - `config()`: 获取全局配置的函数
/// {% endif %}

pub use crate::error::Error;

/// 项目统一的 Result 类型
pub type Result<T> = core::result::Result<T, Error>;

/// 包装结构体 - 用于扩展类型的功能
pub struct W<T>(pub T);

impl<T> std::fmt::Debug for W<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::fmt::Display for W<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Clone for W<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        W(self.0.clone())
    }
}

impl<T> PartialEq for W<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

{% if use_logging %}
pub use tracing::{debug, error, event, info, instrument, span, trace, warn, Level};
{% endif %}

{% if use_config %}
pub use crate::optional::config::{AppConfig, config};
{% endif %}

#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        $crate::error::Error::Generic($msg.to_string())
    };
    ($fmt:expr, $($arg:expr),*) => {
        $crate::error::Error::Generic(format!($fmt, $($arg),*))
    };
}

{% if use_logging %}
#[macro_export]
macro_rules! log_err {
    ($msg:expr) => {
        {
            $crate::prelude::error!("{}", $msg);
            Err($crate::error::Error::Generic($msg.to_string()))
        }
    };
    ($fmt:expr, $($arg:expr),*) => {
        {
            let msg = format!($fmt, $($arg),*);
            $crate::prelude::error!("{}", msg);
            Err($crate::error::Error::Generic(msg))
        }
    };
}
{% endif %}

pub use crate::err;
{% if use_logging %}
pub use crate::log_err;
{% endif %}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_type() {
        let success: Result<i32> = Ok(42);
        assert!(success.is_ok());

        let failure: Result<i32> = Err(Error::Generic("failed".to_string()));
        assert!(failure.is_err());
    }
}
