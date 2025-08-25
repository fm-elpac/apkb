// apkb_api 工具
package io.github.fm_elpac.apkb.api

import java.io.File
import java.util.Locale

import android.os.Build

import io.github.fm_elpac.azi.Azi

fun getDeviceLocale(): String {
    return Locale.getDefault().toString()
}

fun getDeviceApiLevel(): Int {
    return Build.VERSION.SDK_INT
}

fun writeTextFile(f: File, t: String) {
    try {
        f.writeText(t)
    } catch (e: Exception) {
        e.printStackTrace()
    }
}
