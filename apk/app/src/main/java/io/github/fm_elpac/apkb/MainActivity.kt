package io.github.fm_elpac.apkb

import android.app.Activity
import android.os.Bundle
import android.webkit.WebView
import android.webkit.WebViewClient
import android.webkit.JavascriptInterface

import io.github.fm_elpac.azi.Azi
import io.github.fm_elpac.azi.AziCb
import io.github.fm_elpac.azi.AziWebView

class MainActivity: Activity() {
    var aw: AziWebView? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val w = AziWebView(this)
        // 显示 WebView
        setContentView(w.getWebView())

        // status bar color (black)
        window.statusBarColor = 0xff000000.toInt()

        // 添加 js api
        w.addJsApi("apkb_api", ApkbApi(this))
        w.addJsApi("azi_api", LoaderApi())
        // 显示 ui-loader
        w.loadLoader()

        aw = w
        // 开始 (后台) 初始化
        val cb = object: AziCb {
            override fun ok() {
                aw?.loadSdcard(Azi.AZI_DIR_SDCARD_DATA, "apkb-setup/ui/index.html")
            }
        }
        Azi.initZip("apkb-setup.azi.zip", "apkb-setup", cb)

        // DEBUG
        Azi.log("DEBUG: locale = " + getDeviceLocale())
        Azi.log("DEBUG: android api level = " + getDeviceApiLevel())
    }
}

class LoaderApi() {
    // azi_api.getJsLoadList()
    @JavascriptInterface
    fun getJsLoadList(): List<String> {
        return listOf<String>()
    }

    // azi_api.checkInit()
    @JavascriptInterface
    fun checkInit(): String {
        return "加载中 .. ."
    }
}
