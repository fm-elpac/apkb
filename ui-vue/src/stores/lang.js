import { ref } from "vue";
import { defineStore } from "pinia";

import { getDefaultLang, setLang as configSetLang } from "@/api/config/index";
import { get_platform } from "@/api/apkb_api";

// 界面语言
export const LANG_EN = "en";
export const LANG_ZH = "zh";

// 运行平台
export const PLATFORM_ANDROID = "android";
export const PLATFORM_PC = "pc";

// 界面语言设置
export const useLangStore = defineStore("lang", () => {
  const lang = ref(getDefaultLang());
  const platform = ref(get_platform());

  function setLang(value) {
    lang.value = value;

    configSetLang(value);
  }

  function setPlatform(value) {
    platform.value = value;
  }

  return {
    lang,
    platform,

    setLang,
    setPlatform,
  };
});
