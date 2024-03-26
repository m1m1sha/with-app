use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum N2nError {
    // n2n error
    /// 授权错误
    BadAuth = 0,
    /// 方法错误
    BadType,
    /// 无法访问
    NoAccess,
    /// 命令不存在
    NoCmd,
    /// 文件不存在
    NoFile,
    /// 选项不存在
    NoOptions,
    /// 方式不存在
    NoType,
    /// 只读
    ReadOnly,
    /// 只写
    WriteOnly,
    /// 未实现
    UnImplemented,
    /// 未知命令
    UnknownCmd,
    /// 未知细目
    UnknownTopic,

    /// 边缘节点未启动
    EdgeIsStopped,
    /// 边缘节点启动失败
    EdgeStartFailed,

    /// 未知错误
    Unknown,

    // UDP socket error
    /// UDP端口已被使用
    SocketAddrInUse,
    /// UDP连接超时
    SocketConnectTimeout,
    /// UDP连接失败
    SocketConnectFailed,
    /// UDP发送失败
    SocketSendFailed,
    /// UDP接收失败
    SocketRecvFailed,
    /// UDP读等待超时
    SocketReadableTimeout,
    /// UDP解析数据失败
    SocketParseFailed,

    // action error
    /// action 接收失败
    ActionChannelRecvFailed,
    /// action 接收通道已关闭
    ActionChannelRecvClosed,
    /// action 发送通道已满
    ActionChannelSendFull,
    /// action 发送失败
    ActionChannelSendFailed,
    /// action 发送通道已关闭
    ActionChannelSendClosed,

    // arg error
    /// arg 无效数据
    ArgsInvalid,
}
