// apkb_api (js) 接口
package io.github.fm_elpac.apkb.api

import java.io.File

import android.webkit.JavascriptInterface

import kotlinx.serialization.json.Json

import io.github.fm_elpac.azi.Azi

import io.github.fm_elpac.apkb.MainActivity

/// globalThis.apkb_api
class ApkbApi(val a: MainActivity) {
    // apkb_api.get_platform() -> "android"
    @JavascriptInterface
    fun get_platform(): String {
        return "android"
    }

    // apkb_api.get_locale() -> "en_US" | "zh_CN"
    @JavascriptInterface
    fun get_locale(): String {
        return getDeviceLocale()
    }

    // apkb_api.get_android_api_level() -> 31
    @JavascriptInterface
    fun get_android_api_level(): Int {
        return getDeviceApiLevel()
    }

    // apkb_api.debug_log(text)
    @JavascriptInterface
    fun debug_log(text: String) {
        Azi.log("DEBUG: js log: " + text)
    }

    // apkb_api.debug_write_cache(filename, text)
    @JavascriptInterface
    fun debug_write_cache(filename: String, text: String) {
        val f = File(Azi.env(Azi.AZI_DIR_SDCARD_CACHE)!!, filename)
        writeTextFile(f, text)
    }

    // 获取本机安装的 apk 列表
    var pm_get_apk_list_done: Boolean = true
    var pm_apk_list: MutableList<PmGetApkListItem> = mutableListOf()

    // apkb_api.pm_get_apk_list() -> JSON "{ done: true, list: [] }"
    @JavascriptInterface
    fun pm_get_apk_list(): String {
        val r = PmGetApkListResult(pm_get_apk_list_done, pm_apk_list)
        return Json.encodeToString(r)
    }

    // apkb_api.pm_get_apk_list_start()
    @JavascriptInterface
    fun pm_get_apk_list_start() {
        // 启动后台线程
        if (pm_get_apk_list_done) {
            // 未在运行状态
            pm_get_apk_list_done = false;

            val pm = a.packageManager
            Thread(PmGetApkListWorker(this, pm)).start()
        }
    }
}
