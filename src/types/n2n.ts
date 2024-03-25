export interface EdgeArgs {
  /// 组名
  c: string; //
  /// 服务器
  l: string; //
  /// 本地端口
  p: number;
  /// pmtu
  D: boolean;
  /// 本地广播ip
  e: string;
  /// 中继模式
  S: EdgeRelayMode; //
  /// 打洞间隔
  i: number;
  /// 打洞ttl
  L: number;
  /// 组密码
  k: string; //
  /// 加密模式
  A: EdgeEncryptMode; //
  /// 报头加密
  H: boolean;
  /// 压缩模式
  Z: EdgeZipMode;
  /// 基于往返时间选择服务节点
  selectRtt: boolean;
  /// 基于mac选择服务节点
  selectMac: boolean;

  /// 虚拟ip
  a: string;
  /// mac
  m: string;
  /// 网卡名称
  d: string;
  /// mtu
  M: number;
  /// 通过组转发数据
  r: boolean;
  /// 接受多播
  E: boolean; //
  /// 设备描述|昵称
  I: string; //
  /// 服务节点认证密码
  J: string; //
  /// 节点验证公钥
  P: string;
  /// 路由规则
  R: string[];
  /// 网卡跃点
  x: number; //

  /// 管理端口
  t: number;
  /// 管理密码
  managerPasswd: string;
  /// 输出日志
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
  Error = "Error",
}
