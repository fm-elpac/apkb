// API: globalThis.apkb_api

// 返回 globalThis.apkb_api
export function api() {
  return globalThis.apkb_api;
}

// 返回当前运行的平台: "android" | "pc"
export function get_platfrom() {
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

// TODO
