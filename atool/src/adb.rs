use std::{
    env,
    fs::File,
    io::stdout,
    net::{Ipv4Addr, SocketAddrV4},
    process::ExitCode,
};

use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice, DeviceShort};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::cli::bad_cli_arg;

/// 命令 atool adb
///
/// 默认连接: `tcp:127.0.0.1:5037`
///
/// 支持环境变量:
/// + `ADB_PATH` 指定 adb 命令 (二进制文件路径)
/// + `ADB_SERVER_SOCKET` 指定 adb server 连接地址, 比如 `tcp:127.0.0.1:5037`
pub fn c_adb(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() < 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    // 第 1 个参数: 命令
    let r: Vec<String> = (&a[1..]).into();
    match a[0].as_str() {
        "devices" => c_adb_devices(r),
        "pull" => c_adb_pull(r),
        "shell" => c_adb_shell(r),

        _ => {
            bad_cli_arg();
            Err(ExitCode::from(1))
        }
    }
}

const DEFAULT_SERVER_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const DEFAULT_SERVER_PORT: u16 = 5037;

/// 获取 adb server
fn adb_server() -> ADBServer {
    // 默认地址
    let addr1 = SocketAddrV4::new(DEFAULT_SERVER_IP, DEFAULT_SERVER_PORT);
    // 环境变量
    let addr = env::var("ADB_SERVER_SOCKET")
        .map(|s| {
            s.strip_prefix("tcp:")
                .map(|x| x.parse().ok())
                .flatten()
                .unwrap_or_else(|| {
                    eprintln!("ERROR: bad ADB_SERVER_SOCKET={}", s);
                    addr1
                })
        })
        .unwrap_or(addr1);
    // 修复 adb_client 无法禁止启动 adb 的问题
    let adb_path = env::var("ADB_PATH").unwrap_or("adb".into());

    ADBServer::new_from_path(addr, Some(adb_path))
}

/// 获取 adb device
fn adb_device(server: &mut ADBServer) -> ADBServerDevice {
    server.get_device().unwrap()
}

/// adb 设备信息 (JSON)
#[derive(Serialize, Deserialize)]
struct DeviceInfo {
    pub id: String,
    pub state: String,
}

impl From<&DeviceShort> for DeviceInfo {
    fn from(value: &DeviceShort) -> Self {
        Self {
            id: value.identifier.clone(),
            state: format!("{}", value.state),
        }
    }
}

/// 命令 atool adb devices
fn c_adb_devices(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() > 0 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }

    let mut server = adb_server();
    let d = server.devices().unwrap();

    // JSON
    let list: Vec<DeviceInfo> = d.iter().map(|i| i.into()).collect();
    let o = to_string_pretty(&list).unwrap();
    println!("{}", o);

    Ok(())
}

/// 命令 atool adb pull
fn c_adb_pull(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (output, path) = (&a[0], &a[1]);

    let mut server = adb_server();
    let mut device = adb_device(&mut server);

    let mut o = File::create(output).unwrap();
    device.pull(path, &mut o).unwrap();

    Ok(())
}

/// 命令 atool adb shell
fn c_adb_shell(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() < 1 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let mut server = adb_server();
    let mut device = adb_device(&mut server);

    let a1: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    device.shell_command(&a1, &mut stdout()).unwrap();

    Ok(())
}
