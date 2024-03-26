export interface EdgeArgs {
  /** 组名 */
  c: string; //
  /** 服务器 */
  l: string; //
  /** 本地端口 */
  p: number;
  /** pmtu */
  D: boolean;
  /** 本地广播ip */
  e: string;
  /** 中继模式 */
  S: EdgeRelayMode; //
  /** 打洞间隔 */
  i: number;
  /** 打洞ttl */
  L: number;
  /** 组密码 */
  k: string; //
  /** 加密模式 */
  A: EdgeEncryptMode; //
  /** 报头加密 */
  H: boolean;
  /** 压缩模式 */
  Z: EdgeZipMode;
  /** 基于往返时间选择服务节点 */
  selectRtt: boolean;
  /** 基于mac选择服务节点 */
  selectMac: boolean;

  /** 虚拟ip */
  a: string;
  /** mac */
  m: string;
  /** 网卡名称 */
  d: string;
  /** mtu */
  M: number;
  /** 通过组转发数据 */
  r: boolean;
  /** 接受多播 */
  E: boolean; //
  /** 设备描述|昵称 */
  I: string; //
  /** 服务节点认证密码 */
  J: string; //
  /** 节点验证公钥 */
  P: string;
  /** 路由规则 */
  R: string[];
  /** 网卡跃点 */
  x: number; //

  /** 管理端口 */
  t: number;
  /** 管理密码 */
  managerPasswd: string;
  /** 输出日志 */
  v: EdgeTraceMode;
}

export enum EdgeRelayMode {
  None,
  S1,
  S2,
}

export enum EdgeEncryptMode {
  A1,
  A2,
  A3,
  A4,
  A5,
}

export enum EdgeZipMode {
  None,
  Z1,
  Z2,
}

export enum EdgeTraceMode {
  Normal,
  Detailed,
  Brief,
}

export interface EdgeRespStatus {
  running: boolean;
}

export interface EdgeRespCommunity {
  name: string;
}

export interface EdgeRespEdges {
  desc: string;
  ipv4: string;
  lastP2p: number;
  lastSeen: number;
  lastSentQuery: number;
  local: boolean;
  mac: string;
  mode: string;
  purgeable: boolean;
  addr: string;
}

export interface EdgeRespPacketStats {
  p2p: EdgeRespPkt;
  supernode: EdgeRespPkt;
  supernodeBroadcast: EdgeRespPkt;
  transport: EdgeRespPkt;
}

export interface EdgeRespPkt {
  rxPkt: number;
  txPkt: number;
}

export interface EdgeRespSupernodes {
  current: boolean;
  lastSeen: number;
  mac: string;
  purgeable: boolean;
  selection: string;
  addr: string; // ip
  uptime: number;
  version: string;
}
export interface EdgeRespTimestamps {
  lastP2p: number;
  lastSuper: number;
  startTime: number;
}

export interface EdgeRespVerbose {
  traceLevel: number;
}

export enum EdgeFlag {
  Stop = "Stop",
  Status = "Status",
  Verbose = "Verbose",
  Timestamps = "Timestamps",
  Community = "Timestamps",
  SupernodeInfo = "SupernodeInfo",
  PacketStats = "PacketStats",
  EdgeInfo = "EdgeInfo",
}

export enum N2nError {
  // n2n error
  /** 授权错误 */
  BadAuth = 0,
  /** 方法错误 */
  BadType,
  /** 无法访问 */
  NoAccess,
  /** 命令不存在 */
  NoCmd,
  /** 文件不存在 */
  NoFile,
  /** 选项不存在 */
  NoOptions,
  /** 方式不存在 */
  NoType,
  /** 只读 */
  ReadOnly,
  /** 只写 */
  WriteOnly,
  /** 未实现 */
  UnImplemented,
  /** 未知命令 */
  UnknownCmd,
  /** 未知细目 */
  UnknownTopic,

  /** 边缘节点未启动 */
  EdgeIsStopped,
  /** 边缘节点启动失败 */
  EdgeStartFailed,

  /** 未知错误 */
  Unknown,

  // UDP socket error
  /** UDP端口已被使用 */
  SocketAddrInUse,
  /** UDP连接超时 */
  SocketConnectTimeout,
  /** UDP连接失败 */
  SocketConnectFailed,
  /** UDP发送失败 */
  SocketSendFailed,
  /** UDP接收失败 */
  SocketRecvFailed,
  /** UDP读等待超时 */
  SocketReadableTimeout,
  /** UDP解析数据失败 */
  SocketParseFailed,

  // action error
  /** action 接收失败 */
  ActionChannelRecvFailed,
  /** action 接收通道已关闭 */
  ActionChannelRecvClosed,
  /** action 发送通道已满 */
  ActionChannelSendFull,
  /** action 发送失败 */
  ActionChannelSendFailed,
  /** action 发送通道已关闭 */
  ActionChannelSendClosed,

  // arg error
  /** arg 无效数据 */
  ArgsInvalid,
}
