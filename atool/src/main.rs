#![deny(unsafe_code)]

use std::process::ExitCode;

use pm_bin::{cli_arg, init_env_logger, pm_init};
pm_init!();

mod adb;
mod apk;
mod cli;
mod hash;
mod http;
mod sign_info;

fn main() -> Result<(), ExitCode> {
    init_env_logger();

    // 命令行参数解析处理
    if let Some(a) = cli_arg(print_version) {
        if a.len() > 0 {
            // 第 1 个参数: 命令
            let r: Vec<String> = (&a[1..]).into();
            match a[0].as_str() {
                "--help" => {
                    cli::help_en();
                    Ok(())
                }
                "--帮助" => {
                    cli::help_zh();
                    Ok(())
                }

                "sha256" => hash::c_sha256(r),
                "get" => http::c_get(r),
                "manifest" => apk::c_manifest(r),
                "signinfo" => apk::c_signinfo(r),

                "adb" => adb::c_adb(r),

                _ => {
                    eprintln!("ERROR: Bad command `{}`, try --help", a[0]);
                    Err(ExitCode::from(1))
                }
            }
        } else {
            eprintln!("ERROR: Bad command, try --help");
            Err(ExitCode::from(1))
        }
    } else {
        // pm-bin 会处理 `--version` 和 `--版本`
        Ok(())
    }
}
