use std::{fs::File, io::Write, process::ExitCode};

use apksig::Apk;
use rusty_axml::parse_from_apk;
use serde_json::to_string_pretty;

use crate::{cli::bad_cli_arg, sign_info::SignInfo};

/// 命令 atool manifest FILE OUTPUT
pub fn c_manifest(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (f, o) = (&a[0], &a[1]);

    // manifest.xml
    let m = parse_from_apk(f).unwrap();
    let xml = m.to_string().unwrap();

    // `-` 表示 stdout
    if "-" == o {
        println!("{}", xml);
    } else {
        let mut r = File::create(o).unwrap();
        r.write_all(xml.as_bytes()).unwrap();
    }
    Ok(())
}

/// 命令 atool signinfo FILE OUTPUT
pub fn c_signinfo(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (f, o) = (&a[0], &a[1]);

    // 读取 apk 签名数据
    let a = Apk::new(f.into()).unwrap();
    let s = a.get_signing_block().unwrap();

    // JSON
    let info: SignInfo = s.into();
    let json = to_string_pretty(&info).unwrap();
    // `-` 表示 stdout
    if "-" == o {
        println!("{}", json);
    } else {
        let mut r = File::create(o).unwrap();
        r.write_all(json.as_bytes()).unwrap();
    }
    Ok(())
}
