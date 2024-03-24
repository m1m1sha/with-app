use serde::{ser::Serializer, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum N2nError {
    #[error("授权错误")]
    BadAuth,
    #[error("方法错误")]
    BadType,
    #[error("无法访问")]
    NoAccess,
    #[error("命令不存在")]
    NoCmd,
    #[error("文件不存在")]
    NoFile,
    #[error("选项不存在")]
    NoOptions,
    #[error("方式不存在")]
    NoType,
    #[error("只读")]
    ReadOnly,
    #[error("只写")]
    WriteOnly,
    #[error("未实现")]
    UnImplemented,
    #[error("未知命令")]
    UnknownCmd,
    #[error("未知细目")]
    UnknownTopic,

    #[error("地址已被使用")]
    AddrInUse,
    #[error("超时")]
    Timeout,
    #[error("连接失败")]
    ConnectFailed,
    #[error("发送失败")]
    SendFailed,
    #[error("等待读取失败")]
    Readable,
    #[error("解析失败")]
    Parse,
    #[error("无法校验数据")]
    InvalidData,
    #[error("未知错误")]
    Unknown,
}

impl Serialize for N2nError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
