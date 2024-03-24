use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RespError {
    /// 授权错误
    BadAuth,
    /// 错误的方法 action: r|w
    BadType,

    /// 命令不存在
    NoCmd,
    /// 文件不存在
    NoFile,
    /// 选项不存在
    NoOptions,
    /// 方法不存在
    NoType,

    /// 只读
    ReadOnly,
    /// 只写
    WriteOnly,

    /// 未实现
    UnImplemented,
    /// 未知命令
    UnknownCmd,
}

impl RespError {
    pub fn from(str: &str) -> Self {
        // 正常情况下不可能有其他错误类型
        match str {
            "badauth" => RespError::BadAuth,
            "badtype" => RespError::BadType,
            "nocmd" => RespError::NoCmd,
            "nofile" => RespError::NoFile,
            "nooptions" => RespError::NoOptions,
            "notype" => RespError::NoType,
            "readonly" => RespError::ReadOnly,
            "writeonly" => RespError::WriteOnly,
            "unknowncmd" => RespError::UnknownCmd,
            _ => RespError::UnImplemented,
        }
    }
}
