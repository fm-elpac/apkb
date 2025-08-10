use std::{
    fs::File,
    io::{Read, Write},
    process::ExitCode,
};

use sha2::{Digest, Sha256};

use crate::cli::bad_cli_arg;

/// 命令 atool sha256
pub fn c_sha256(a: Vec<String>) -> Result<(), ExitCode> {
    // 解析命令行参数
    if a.len() != 2 {
        bad_cli_arg();
        return Err(ExitCode::from(1));
    }
    let (f, o) = (&a[0], &a[1]);

    // 输入文件
    let mut i = File::open(f).unwrap();

    // 读取缓冲区
    let mut b = [0; 2048];
    // 计算 hash
    let mut h = Sha256::new();

    loop {
        let l = i.read(&mut b).unwrap();
        if 0 == l {
            break;
        }

        h.update(&b[0..l]);
    }
    // hex 结果
    let h1 = h.finalize();
    let hh = base16ct::lower::encode_string(&h1);

    let mut r = File::create(o).unwrap();
    r.write_all(hh.as_bytes()).unwrap();

    Ok(())
}
