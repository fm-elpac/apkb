package io.github.fm_elpac.apkb

import android.app.Application

import io.github.fm_elpac.azi.Azi

class ApkbApp: Application() {

    override fun onCreate() {
        super.onCreate()

        // init azi
        Azi.setContext(this)
    }
}
