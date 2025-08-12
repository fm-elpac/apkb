# atool

![CI](https://github.com/fm-elpac/apkb/actions/workflows/ci.yml/badge.svg)

Little tool for Android apk (adb).

---

CLI usage:

```sh
> ./atool --help
atool: Little tool for Android apk (adb).
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

More info: <https://github.com/fm-elpac/apkb> <https://crates.io/crates/atool>
```

命令行参数:

```sh
> ./atool --帮助
atool: 用于 Android apk (adb) 的小工具.
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

更多信息: <https://github.com/fm-elpac/apkb> <https://crates.io/crates/atool>
```

## adb 问题

由于依赖的 `adb_client` 在连接本地 adb 之前会尝试启动 adb server
(目前无法关闭这一行为), 如果想避免运行 `adb start-server`, 建议使用如下方式运行:

```sh
env ADB_PATH=/usr/bin/true ./atool adb devices
```

## 编译 (本地 ArchLinux)

- `x86_64-unknown-linux-gnu`

  ```sh
  cargo build --release
  ```

- `aarch64-linux-android`

  ```sh
  rustup target add aarch64-linux-android

  export ANDROID_NDK_HOME=~/Android/Sdk/ndk/28.1.13356709
  export ANDROID_NDK_ROOT=~/Android/Sdk/ndk/28.1.13356709
  make atool
  ```

- `aarch64-unknown-linux-gnu`

  ```sh
  sudo pacman -S aarch64-linux-gnu-glibc
  rustup target add aarch64-unknown-linux-gnu

  export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
  export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
  export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
  cargo build --release --target aarch64-unknown-linux-gnu
  ```

- `riscv64gc-unknown-linux-gnu`

  ```sh
  sudo pacman -S riscv64-linux-gnu-glibc
  rustup target add riscv64gc-unknown-linux-gnu

  export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc
  export CC_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-gcc
  export CXX_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-g++
  cargo build --release --target riscv64gc-unknown-linux-gnu
  ```

## LICENSE

GNU General Public License v3.0 or later (SPDX Identifier: `GPL-3.0-or-later`)

<https://spdx.org/licenses/GPL-3.0-or-later.html>
