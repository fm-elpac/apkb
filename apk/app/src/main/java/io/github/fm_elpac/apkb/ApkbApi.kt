package io.github.fm_elpac.apkb

import java.util.Locale

import android.os.Build
import android.webkit.JavascriptInterface

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
}

fun getDeviceLocale(): String {
    return Locale.getDefault().toString()
}

fun getDeviceApiLevel(): Int {
    return Build.VERSION.SDK_INT
}
