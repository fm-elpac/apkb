use std::process::ExitCode;

use crate::cli::bad_cli_arg;

/// 命令 atool manifest
pub fn c_manifest(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (file, output) = (&a[0], &a[1]);
    println!("TODO manifest {} {}", file, output);

    // TODO
    Ok(())
}

/// 命令 atool signinfo
pub fn c_signinfo(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (file, output) = (&a[0], &a[1]);
    println!("TODO signinfo {} {}", file, output);

    // TODO
    Ok(())
}
