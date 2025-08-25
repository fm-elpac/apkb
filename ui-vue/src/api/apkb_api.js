// API: globalThis.apkb_api

// 返回 globalThis.apkb_api
export function api() {
  return globalThis.apkb_api;
}

// 返回当前运行的平台: "android" | "pc"
export function get_platform() {
  return api()?.get_platform();
}

// 返回当前 (设备) 语言设置: "en_US" | "zh_CN"
export function get_locale() {
  return api()?.get_locale();
}

// 返回 Android 版本 (api level), 比如: 31
export function get_android_api_level() {
  return api()?.get_android_api_level();
}

// 用于调试
export function debug_log(text) {
  console.log(text);
  api()?.debug_log(text);
}

// 用于调试
export function debug_write_cache(filename, text) {
  api()?.debug_write_cache(filename, text);
}

// 获取本机安装的 apk 列表: 启动后台线程
export function pm_get_apk_list_start() {
  api()?.pm_get_apk_list_start();
}

// 获取本机安装的 apk 列表: 获取结果
export function pm_get_apk_list() {
  const r = api()?.pm_get_apk_list();
  if (null != r) {
    return JSON.parse(r);
  }
}

// TODO
