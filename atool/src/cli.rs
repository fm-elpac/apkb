/// 显示命令行帮助信息

/// --help
pub fn help_en() {
    println!(
        r#"atool: Little tool for Android apk (adb).
Usage: atool COMMAND ARG..

atool sha256 FILE OUTPUT.sha256
    Calculate sha256 of a file.

atool get OUTPUT URL
    Download file from URL (HTTP).

atool manifest FILE.apk OUTPUT.xml
    Extract Android manifest.xml from a apk.

atool signinfo FILE.apk OUTPUT.json
    Extract signature info from a apk.

----
atool adb COMMAND..
    Some adb command (only subset function).

atool adb devices
    List attached adb devices.

atool adb pull OUTPUT PATH
    Download a file from the adb device.

atool adb shell COMMAND..
    Run a shell command on the adb device.

----
atool --version
    Show version info.

atool --help
    Show this help info.

atool --版本
    Show version info.

atool --帮助
    Show help info (Chinese).

More info: <https://github.com/fm-elpac/apkb> <https://crates.io/crates/atool>"#
    );
}

/// --帮助
pub fn help_zh() {
    println!(
        r#"atool: 用于 Android apk (adb) 的小工具.
用法: atool 命令 参数..

atool sha256 文件 输出.sha256
    计算一个文件的 sha256.

atool get 输出 URL
    从 URL (HTTP) 下载文件.

atool manifest 文件.apk 输出.xml
    从 apk 提取 Android 清单文件 (manifest.xml).

atool signinfo 文件.apk 输出.json
    从 apk 提取签名信息.

----
atool adb 命令..
    一些 adb 命令 (仅支持部分功能).

atool adb devices
    列出连接的 adb 设备 (比如 手机).

atool adb pull 输出 路径
    从 adb 设备下载文件.

atool adb shell 命令..
    在 adb 设备上运行 shell 命令.

----
atool --版本
    显示版本信息.

atool --帮助
    显示此帮助信息.

atool --version
    显示版本信息.

atool --help
    显示帮助信息 (英文).

更多信息: <https://github.com/fm-elpac/apkb> <https://crates.io/crates/atool>"#
    );
}

/// 命令行参数错误信息
pub fn bad_cli_arg() {
    eprintln!("ERROR: Bad command arg, try --help");
}
