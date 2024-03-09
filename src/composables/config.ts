export enum cipherMode {
  AesGcm = 0,
  AesCbc,
  AesEcb,
  Sm4Cbc,
  None,
}

export enum punchMode {
  IPv4 = 0,
  IPv6,
  All,
}

export enum channelMode {
  Relay = 0,
  P2p,
  All,
}

export interface withConfig {
  // udi: String,                        // 设备唯一标识
  server: string // withs 节点
  token: string // 组网 token | 房间名
  passwd: string // 组网密码

  stuns: string[] // stun 节点
  name: string // 组网昵称
  proxy: boolean // 内置代理
  server_encrypt: boolean // 服务端加密
  finger: boolean // 指纹
  latency: boolean // 延迟优先
  // ports: number[]; // 端口
  // mtu: number; // mtu
  tcp: boolean // 强制tcp
  ip: string // 自定义 ip
  // inbound: Vec<(u32, u32, Ipv4Addr)>, // 入站ip
  // outbound: Vec<(u32, u32)>,          // 出站ip
  parallel: number // 处理协程数
  cipher: cipherMode // 加密模式
  punch: punchMode // 打洞模式
  channel: channelMode // 信道模式
}

export const DEFAULT_CONFIG: config = {
  with: {
    server: 'nat1.wherewego.top:29872',
    token: '',
    passwd: '',
    stuns: [
      'stun.qq.com:3478',
      'stun.miwifi.com:3478',
      'stun1.l.google.com:19302',
      'stun2.l.google.com:19302',
    ],
    name: '',
    proxy: true,
    server_encrypt: false,
    finger: false,
    latency: true,
    tcp: false,
    ip: '',
    parallel: 2,
    cipher: cipherMode.None,
    punch: punchMode.All,
    channel: channelMode.All,
  },
  servers: [],
}
