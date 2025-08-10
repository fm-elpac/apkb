use std::{fs::File, process::ExitCode};

use reqwest::blocking::Client;

use crate::cli::bad_cli_arg;

/// 命令 atool get
pub fn c_get(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (output, url) = (&a[0], &a[1]);

    let client = Client::new();
    let mut r = client.get(url).send().unwrap();

    let mut o = File::create(output).unwrap();
    r.copy_to(&mut o).unwrap();

    Ok(())
}
