/// 应用错误类型
///
/// 使用 thiserror 自动实现 Error trait
/// 用户应根据业务需求自行添加错误变体
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 通用错误
    #[error("Error: {0}")]
    Generic(String),

    /// IO 错误（自动转换）
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl Error {
    pub fn generic(msg: impl Into<String>) -> Self {
        Error::Generic(msg.into())
    }
}
