package io.github.fm_elpac.apkb

import java.io.File
import java.io.FileOutputStream
import java.util.Locale

import android.content.pm.ApplicationInfo
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.drawable.BitmapDrawable
import android.graphics.drawable.Drawable
import android.os.Build
import android.webkit.JavascriptInterface

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

import io.github.fm_elpac.azi.Azi

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

fun getDeviceLocale(): String {
    return Locale.getDefault().toString()
}

fun getDeviceApiLevel(): Int {
    return Build.VERSION.SDK_INT
}

@Serializable
data class PmGetApkListItem(
    // packageName, applicationId
    var id: String,
    // 整数版本号
    var versionCode: Int,
    // 文本版本号
    var versionName: String?,

    // 名称 (当前语言)
    var label: String,
    // 图标 (本地文件路径)
    var icon: String?,
    // 兼容的最低 Android 版本 (api level)
    var minSdk: Int,
    // apk 文件路径
    var apk: String,
)

@Serializable
data class PmGetApkListResult(
    var done: Boolean,
    var list: List<PmGetApkListItem>,
)

// 后台线程: 获取本机安装的 apk 列表
class PmGetApkListWorker(val a: ApkbApi, val pm: PackageManager): Runnable {

    private fun work() {
        val list = pm.getInstalledPackages(PackageManager.GET_META_DATA)

        // 保存原始信息, 供查找
        val m: MutableMap<String, PackageInfo> = mutableMapOf()

        // 清除结果集
        a.pm_apk_list = mutableListOf()
        for (i in list) {
            val x = i.applicationInfo
            if (null == x) {
                continue
            }
            // 忽略系统应用
            if ((x.flags and ApplicationInfo.FLAG_SYSTEM) != 0) {
                continue
            }
            if ((x.flags and ApplicationInfo.FLAG_UPDATED_SYSTEM_APP) != 0) {
                continue
            }
            m[i.packageName] = i

            a.pm_apk_list.add(PmGetApkListItem(
                i.packageName,
                i.versionCode,
                i.versionName,

                x.loadLabel(pm).toString(),
                null,
                x.minSdkVersion,
                x.sourceDir,
            ))
        }

        // 图片缓存路径: AZI_DIR_SDCARD_CACHE/icon
        val c = File(Azi.env(Azi.AZI_DIR_SDCARD_CACHE)!!, "icon")
        c.mkdirs()
        // 获取应用图标
        for (i in a.pm_apk_list) {
            val x = m[i.id]!!.applicationInfo!!
            val d = x.loadIcon(pm)

            // 保存图标至本地缓存
            val p = File(c, i.id + ".png")
            save_drawable(d, p)

            i.icon = p.absolutePath
        }
    }

    override fun run() {
        // DEBUG
        Azi.log("DEBUG: PmGetApkListWorker.run()")

        // 出错不允许崩溃
        try {
            work()
        } catch (e: Exception) {
            e.printStackTrace()
        }

        a.pm_get_apk_list_done = true
        // DEBUG
        Azi.log("DEBUG: PmGetApkListWorker.run() end")
    }
}

// 保存图片为 png
fun save_drawable(d: Drawable, f: File) {
    try {
        val o = FileOutputStream(f)
        var b: Bitmap
        if (d is BitmapDrawable) {
            b = d.getBitmap()
        } else {
            // 绘制图标
            b = Bitmap.createBitmap(d.intrinsicWidth, d.intrinsicHeight, Bitmap.Config.ARGB_8888)
            val c = Canvas(b)
            d.setBounds(0, 0, c.width, c.height)
            d.draw(c)
        }
        b.compress(Bitmap.CompressFormat.PNG, 100, o)
        o.close()
    } catch (e: Exception) {
        e.printStackTrace()
    }
}
