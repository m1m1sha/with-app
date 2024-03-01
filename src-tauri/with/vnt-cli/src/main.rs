use std::io;
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;

use console::style;
use getopts::Options;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::signal;

use common::args_parse::{ips_parse, out_ips_parse};
use vnt::channel::punch::PunchModel;
use vnt::cipher::CipherModel;
use vnt::core::{Config, Vnt, VntUtil};
use vnt::handle::handshake_handler::HandshakeEnum;
use vnt::handle::registration_handler::ReqEnum;

mod command;
mod config;
mod console_out;
mod root_check;

pub fn app_home() -> io::Result<PathBuf> {
    let path = std::env::current_exe()
        .expect("current_exe error")
        .parent()
        .expect("current_exe parent error")
        .join("env");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn main() {
    let _ = log4rs::init_file("log4rs.yaml", Default::default());
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("k", "", "组网标识", "<token>");
    opts.optopt("n", "", "设备名称", "<name>");
    opts.optopt("d", "", "设备标识", "<id>");
    opts.optflag("c", "", "关闭交互式命令");
    opts.optopt("s", "", "注册和中继服务器地址", "<server>");
    opts.optmulti("e", "", "stun服务器", "<stun-server>");
    opts.optflag("a", "", "使用tap模式");
    opts.optmulti("i", "", "配置点对网(IP代理)入站时使用", "<in-ip>");
    opts.optmulti("o", "", "配置点对网出站时使用", "<out-ip>");
    opts.optopt("w", "", "客户端加密", "<password>");
    opts.optflag("W", "", "服务端加密");
    opts.optflag("m", "", "模拟组播");
    opts.optopt("u", "", "自定义mtu(默认为1430)", "<mtu>");
    opts.optflag("", "tcp", "tcp");
    opts.optopt("", "ip", "指定虚拟ip", "<ip>");
    opts.optflag("", "relay", "仅使用服务器转发");
    opts.optopt("", "par", "任务并行度(必须为正整数)", "<parallel>");
    opts.optopt("", "thread", "线程数(必须为正整数)", "<thread>");
    opts.optopt("", "model", "加密模式", "<model>");
    opts.optflag("", "finger", "指纹校验");
    opts.optopt("", "punch", "取值ipv4/ipv6", "<punch>");
    opts.optopt("", "port", "监听的端口", "<port>");
    opts.optflag("", "cmd", "开启窗口输入");
    opts.optflag("", "no-proxy", "关闭内置代理");
    opts.optflag("", "first-latency", "优先延迟");
    opts.optopt("f", "", "配置文件", "<conf>");
    //"后台运行时,查看其他设备列表"
    opts.optflag("", "list", "后台运行时,查看其他设备列表");
    opts.optflag("", "all", "后台运行时,查看其他设备完整信息");
    opts.optflag("", "info", "后台运行时,查看当前设备信息");
    opts.optflag("", "route", "后台运行时,查看数据转发路径");
    opts.optflag("", "stop", "停止后台运行");
    opts.optflag("h", "help", "帮助");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&program, opts);
            println!("{}", f.to_string());
            return;
        }
    };
    if matches.opt_present("h") || args.len() == 1 {
        print_usage(&program, opts);
        return;
    }
    if !root_check::is_app_elevated() {
        println!("Please run it with administrator or root privileges");
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        sudo::escalate_if_needed().unwrap();
        return;
    }
    if matches.opt_present("list") {
        command::command(command::CommandEnum::List);
        return;
    } else if matches.opt_present("info") {
        command::command(command::CommandEnum::Info);
        return;
    } else if matches.opt_present("stop") {
        command::command(command::CommandEnum::Stop);
        return;
    } else if matches.opt_present("route") {
        command::command(command::CommandEnum::Route);
        return;
    } else if matches.opt_present("all") {
        command::command(command::CommandEnum::All);
        return;
    }
    let conf = matches.opt_str("f");
    let (config, cmd) = if conf.is_some() {
        match config::read_config(&conf.unwrap()) {
            Ok(c) => c,
            Err(e) => {
                println!("conf err {}", e);
                return;
            }
        }
    } else {
        if !matches.opt_present("k") {
            print_usage(&program, opts);
            println!("parameter -k not found .");
            return;
        }
        let tap = matches.opt_present("a");
        let token: String = matches.opt_get("k").unwrap().unwrap();
        let device_id = matches.opt_get_default("d", String::new()).unwrap();
        let device_id = if device_id.is_empty() {
            config::get_device_id()
        } else {
            device_id
        };
        if device_id.is_empty() {
            print_usage(&program, opts);
            println!("parameter -d not found .");
            return;
        }
        let name = matches
            .opt_get_default("n", os_info::get().to_string())
            .unwrap();
        let server_address_str = matches
            .opt_get_default("s", "nat1.wherewego.top:29872".to_string())
            .unwrap();
        let server_address = match server_address_str.to_socket_addrs() {
            Ok(mut addr) => {
                if let Some(addr) = addr.next() {
                    addr
                } else {
                    println!("parameter '-s {}' error .", server_address_str);
                    return;
                }
            }
            Err(e) => {
                println!("parameter '-s {}' error {}.", server_address_str, e);
                return;
            }
        };
        let mut stun_server = matches.opt_strs("e");
        if stun_server.is_empty() {
            stun_server.push("stun1.l.google.com:19302".to_string());
            stun_server.push("stun2.l.google.com:19302".to_string());
            stun_server.push("stun.qq.com:3478".to_string());
        }

        let in_ip = matches.opt_strs("i");
        let in_ip = match ips_parse(&in_ip) {
            Ok(in_ip) => in_ip,
            Err(e) => {
                print_usage(&program, opts);
                println!();
                println!("-i: {:?} {}", in_ip, e);
                println!("example: -i 192.168.0.0/24,10.26.0.3");
                return;
            }
        };
        let out_ip = matches.opt_strs("o");
        let out_ip = match out_ips_parse(&out_ip) {
            Ok(out_ip) => out_ip,
            Err(e) => {
                print_usage(&program, opts);
                println!();
                println!("-o: {:?} {}", out_ip, e);
                println!("example: -o 0.0.0.0/0");
                return;
            }
        };
        let password: Option<String> = matches.opt_get("w").unwrap();
        let server_encrypt = matches.opt_present("W");
        #[cfg(not(feature = "server_encrypt"))]
        {
            if server_encrypt {
                println!("Server encryption not supported");
                return;
            }
        }
        let simulate_multicast = matches.opt_present("m");
        let mtu: Option<String> = matches.opt_get("u").unwrap();
        let mtu = if let Some(mtu) = mtu {
            match u16::from_str(&mtu) {
                Ok(mtu) => Some(mtu),
                Err(e) => {
                    print_usage(&program, opts);
                    println!();
                    println!("'-u {}' {}", mtu, e);
                    return;
                }
            }
        } else {
            None
        };
        let virtual_ip: Option<String> = matches.opt_get("ip").unwrap();
        let virtual_ip =
            virtual_ip.map(|v| Ipv4Addr::from_str(&v).expect(&format!("'--ip {}' error", v)));
        if let Some(virtual_ip) = virtual_ip {
            if virtual_ip.is_unspecified() || virtual_ip.is_broadcast() || virtual_ip.is_multicast()
            {
                println!("'--ip {}' invalid", virtual_ip);
                return;
            }
        }
        let tcp_channel = matches.opt_present("tcp");
        let relay = matches.opt_present("relay");
        let parallel = matches.opt_get::<usize>("par").unwrap().unwrap_or(1);
        if parallel == 0 {
            println!("'--par {}' invalid", parallel);
            return;
        }

        let cipher_model = match matches.opt_get::<CipherModel>("model") {
            Ok(model) => {
                #[cfg(not(any(
                    feature = "aes_gcm",
                    feature = "server_encrypt",
                    feature = "aes_cbc",
                    feature = "aes_ecb",
                    feature = "sm4_cbc"
                )))]
                {
                    if password.is_some() && model.is_none() {
                        println!("Encryption not supported");
                        return;
                    }
                }
                #[cfg(not(any(feature = "aes_gcm", feature = "server_encrypt")))]
                {
                    if password.is_some() && model.is_none() {
                        println!("'--model ' undefined");
                        return;
                    }
                    model.unwrap_or(CipherModel::None)
                }
                #[cfg(any(feature = "aes_gcm", feature = "server_encrypt"))]
                model.unwrap_or(CipherModel::AesGcm)
            }
            Err(e) => {
                println!("'--model ' invalid,{}", e);
                return;
            }
        };

        let finger = matches.opt_present("finger");
        let punch_model = matches
            .opt_get::<PunchModel>("punch")
            .unwrap()
            .unwrap_or(PunchModel::All);
        let port = matches.opt_get::<u16>("port").unwrap_or(None).unwrap_or(0);
        let cmd = matches.opt_present("cmd");
        #[cfg(feature = "ip_proxy")]
        let no_proxy = matches.opt_present("no-proxy");
        let first_latency = matches.opt_present("first-latency");
        let config = Config::new(
            tap,
            token,
            device_id,
            name,
            server_address,
            server_address_str,
            stun_server,
            in_ip,
            out_ip,
            password,
            simulate_multicast,
            mtu,
            tcp_channel,
            virtual_ip,
            relay,
            #[cfg(feature = "ip_proxy")]
            no_proxy,
            server_encrypt,
            parallel,
            cipher_model,
            finger,
            punch_model,
            port,
            first_latency,
        )
        .unwrap();
        (config, cmd)
    };
    println!("version {}", vnt::VNT_VERSION);

    main0(config, cmd);
    std::process::exit(0);
}

#[tokio::main]
async fn main0(config: Config, show_cmd: bool) {
    let server_encrypt = config.server_encrypt;
    let mut vnt_util = VntUtil::new(config).unwrap();
    let mut conn_count = 0;
    let response = loop {
        if conn_count > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        conn_count += 1;
        if let Err(e) = vnt_util.connect() {
            println!("connect server failed {}", e);
            return;
        }
        match vnt_util.handshake() {
            Ok(response) => {
                if server_encrypt {
                    let finger = response.unwrap().finger().unwrap();
                    println!("{}{}", green("server fingerprint:".to_string()), finger);
                    match vnt_util.secret_handshake() {
                        Ok(_) => {}
                        Err(e) => {
                            match e {
                                HandshakeEnum::NotSecret => {}
                                HandshakeEnum::KeyError => {}
                                HandshakeEnum::Timeout => {
                                    println!("handshake timeout")
                                }
                                HandshakeEnum::ServerError(str) => {
                                    println!("error:{}", str);
                                }
                                HandshakeEnum::Other(str) => {
                                    println!("error:{}", str);
                                }
                            }
                            continue;
                        }
                    }
                }
                match vnt_util.register() {
                    Ok(response) => {
                        break response;
                    }
                    Err(e) => match e {
                        ReqEnum::TokenError => {
                            println!("token error");
                            return;
                        }
                        ReqEnum::AddressExhausted => {
                            println!("address exhausted");
                            return;
                        }
                        ReqEnum::Timeout => {
                            println!("timeout...");
                        }
                        ReqEnum::ServerError(str) => {
                            println!("error:{}", str);
                        }
                        ReqEnum::Other(str) => {
                            println!("error:{}", str);
                        }
                        ReqEnum::IpAlreadyExists => {
                            println!("ip already exists");
                            return;
                        }
                        ReqEnum::InvalidIp => {
                            println!("invalid ip");
                            return;
                        }
                    },
                }
            }
            Err(e) => match e {
                HandshakeEnum::NotSecret => {
                    println!("The server does not support encryption");
                    return;
                }
                HandshakeEnum::KeyError => {}
                HandshakeEnum::Timeout => {
                    println!("handshake timeout")
                }
                HandshakeEnum::ServerError(str) => {
                    println!("error:{}", str);
                }
                HandshakeEnum::Other(str) => {
                    println!("error:{}", str);
                }
            },
        }
    };
    println!(" ====== Connect Successfully ====== ");
    println!("virtual_gateway:{}", response.virtual_gateway);
    println!("virtual_ip:{}", green(response.virtual_ip.to_string()));
    let driver_info = vnt_util.create_iface().unwrap();
    println!(" ====== Create Network Interface Successfully ====== ");
    println!("name:{}", driver_info.name);
    println!("version:{}", driver_info.version);
    let mut vnt = match vnt_util.build().await {
        Ok(vnt) => vnt,
        Err(e) => {
            println!("error:{}", e);
            return;
        }
    };
    println!(" ====== Start Successfully ====== ");
    let vnt_c = vnt.clone();
    tokio::spawn(async {
        if let Err(e) = command::server::CommandServer::new().start(vnt_c).await {
            log::warn!("cmd:{:?}", e);
            println!("command error :{}", e);
        }
    });
    if show_cmd {
        let stdin = tokio::io::stdin();
        let mut cmd = String::new();
        let mut reader = BufReader::new(stdin);
        loop {
            cmd.clear();
            println!("input:list,info,route,all,stop");
            tokio::select! {
                _ = vnt.wait_stop()=>{
                    return;
                }
                _ = signal::ctrl_c()=>{
                    let _ = vnt.stop();
                    vnt.wait_stop_ms(std::time::Duration::from_secs(3)).await;
                    std::process::exit(0);
                }
                rs = reader.read_line(&mut cmd)=>{
                     match rs {
                        Ok(len) => {
                            if !command(&cmd[..len],&vnt){
                                break;
                            }
                        }
                        Err(e) => {
                            println!("input err:{}",e);
                            break;
                        }
                    }
                }
            }
        }
    }
    vnt.wait_stop().await;
}

fn command(cmd: &str, vnt: &Vnt) -> bool {
    if cmd.is_empty() {
        return false;
    }
    match cmd.to_lowercase().trim() {
        "list" => {
            let list = command::command_list(&vnt);
            console_out::console_device_list(list);
        }
        "info" => {
            let info = command::command_info(&vnt);
            console_out::console_info(info);
        }
        "route" => {
            let route = command::command_route(&vnt);
            console_out::console_route_table(route);
        }
        "all" => {
            let list = command::command_list(&vnt);
            console_out::console_device_list_all(list);
        }
        "stop" => {
            let _ = vnt.stop();
            return false;
        }
        _ => {}
    }
    println!();
    return true;
}

fn print_usage(program: &str, _opts: Options) {
    println!("Usage: {} [options]", program);
    println!("version:{}", vnt::VNT_VERSION);
    println!("Options:");
    println!(
        "  -k <token>          {}",
        green("使用相同的token,就能组建一个局域网络".to_string())
    );
    println!("  -n <name>           给设备一个名字,便于区分不同设备,默认使用系统版本");
    println!("  -d <id>             设备唯一标识符,不使用--ip参数时,服务端凭此参数分配虚拟ip");
    println!("  -s <server>         注册和中继服务器地址");
    println!("  -e <stun-server>    stun服务器,用于探测NAT类型,可多次指定,如-e addr1 -e addr2");
    println!("  -a                  使用tap模式,默认使用tun模式");
    println!("  -i <in-ip>          配置点对网(IP代理)时使用,-i 192.168.0.0/24,10.26.0.3表示允许接收网段192.168.0.0/24的数据");
    println!("                      并转发到10.26.0.3,可指定多个网段");
    println!("  -o <out-ip>         配置点对网时使用,-o 192.168.0.0/24表示允许将数据转发到192.168.0.0/24,可指定多个网段");
    #[cfg(not(feature = "ip_proxy"))]
    println!("                      注意需要在系统配置ip转发才可正常使用");
    #[cfg(not(any(
        feature = "aes_gcm",
        feature = "server_encrypt",
        feature = "aes_cbc",
        feature = "aes_ecb",
        feature = "sm4_cbc"
    )))]
    let enums = String::new();
    #[cfg(any(
        feature = "aes_gcm",
        feature = "server_encrypt",
        feature = "aes_cbc",
        feature = "aes_ecb",
        feature = "sm4_cbc"
    ))]
    let mut enums = String::new();
    #[cfg(any(feature = "aes_gcm", feature = "server_encrypt"))]
    enums.push_str("/aes_gcm");
    #[cfg(feature = "aes_cbc")]
    enums.push_str("/aes_cbc");
    #[cfg(feature = "aes_ecb")]
    enums.push_str("/aes_ecb");
    #[cfg(feature = "sm4_cbc")]
    enums.push_str("/sm4_cbc");
    if !enums.is_empty() {
        println!("  -w <password>       使用该密码生成的密钥对客户端数据进行加密,并且服务端无法解密,使用相同密码的客户端才能通信");
    }
    #[cfg(feature = "server_encrypt")]
    println!("  -W                  加密当前客户端和服务端通信的数据,请留意服务端指纹是否正确");
    println!("  -m                  模拟组播,默认情况下组播数据会被当作广播发送,开启后会模拟真实组播的数据发送");
    println!("  -u <mtu>            自定义mtu(不加密默认为1450，加密默认为1410)");
    println!("  -f <conf_file>      读取配置文件中的配置");

    println!("  --tcp               和服务端使用tcp通信,默认使用udp,遇到udp qos时可指定使用tcp");
    println!("  --ip <ip>           指定虚拟ip,指定的ip不能和其他设备重复,必须有效并且在服务端所属网段下,默认情况由服务端分配");
    println!("  --relay             仅使用服务器转发,不使用p2p,默认情况允许使用p2p");
    println!("  --par <parallel>    任务并行度(必须为正整数),默认值为1");
    if !enums.is_empty() {
        println!(
            "  --model <model>     加密模式(默认aes_gcm)，可选值{}",
            &enums[1..]
        );
    }
    if !enums.is_empty() {
        println!("  --finger            增加数据指纹校验，可增加安全性，如果服务端开启指纹校验，则客户端也必须开启");
    }
    println!("  --punch <punch>     取值ipv4/ipv6，ipv4表示仅使用ipv4打洞");
    println!("  --port <port>       取值0~65535，指定本地监听的端口，默认取随机端口");
    println!("  --cmd               开启交互式命令，使用此参数开启控制台输入");
    #[cfg(feature = "ip_proxy")]
    println!("  --no-proxy          关闭内置代理，如需点对网则需要配置网卡NAT转发");
    println!("  --first-latency     优先低延迟的通道，默认情况优先使用p2p通道");

    println!();
    println!(
        "  --list              {}",
        yellow("后台运行时,查看其他设备列表".to_string())
    );
    println!(
        "  --all               {}",
        yellow("后台运行时,查看其他设备完整信息".to_string())
    );
    println!(
        "  --info              {}",
        yellow("后台运行时,查看当前设备信息".to_string())
    );
    println!(
        "  --route             {}",
        yellow("后台运行时,查看数据转发路径".to_string())
    );
    println!(
        "  --stop              {}",
        yellow("停止后台运行".to_string())
    );
    println!("  -h, --help          帮助");
}

fn green(str: String) -> impl std::fmt::Display {
    style(str).green()
}

fn yellow(str: String) -> impl std::fmt::Display {
    style(str).yellow()
}
