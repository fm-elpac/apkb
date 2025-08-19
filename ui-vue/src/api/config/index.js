// 配置存储 (localStorage)
import { LANG_EN, LANG_ZH } from "@/stores/lang";

import { api, get_locale } from "../apkb_api";

// 界面语言设置: "zh" | "en"
const KEY_LANG = "apkb.ui_lang";

// 获取存储的语言
export function getLang() {
  return localStorage.getItem(KEY_LANG);
}

// 设置语言
export function setLang(lang) {
  return localStorage.setItem(KEY_LANG, lang);
}

// 获取 默认界面语言: "zh" | "en"
export function getDefaultLang() {
  // 优先使用 用户设置
  let lang = getLang();
  if (null != lang) {
    return lang;
  }

  // 其次使用 apkb_api
  if (api()) {
    lang = get_locale();
    if (lang.startsWith("zh")) {
      return LANG_ZH;
    } else {
      return LANG_EN;
    }
  }

  // 使用浏览器设置
  lang = navigator.language;
  if (lang.startsWith("zh")) {
    return LANG_ZH;
  }

  // 默认值
  return LANG_EN;
}
