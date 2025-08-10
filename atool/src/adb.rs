use std::{
    env,
    fs::File,
    io::stdout,
    net::{Ipv4Addr, SocketAddrV4},
    process::ExitCode,
};

use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice};

use crate::cli::bad_cli_arg;

/// 命令 atool adb
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
    // 修复 adb_client 无法禁止启动 adb 的问题
    let addr = SocketAddrV4::new(DEFAULT_SERVER_IP, DEFAULT_SERVER_PORT);
    let adb_path = env::var("ADB_PATH").unwrap_or("adb".into());

    ADBServer::new_from_path(addr, Some(adb_path))
}

/// 获取 adb device
fn adb_device(server: &mut ADBServer) -> ADBServerDevice {
    server.get_device().unwrap()
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

    // TODO
    println!("{:?}", d);

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
