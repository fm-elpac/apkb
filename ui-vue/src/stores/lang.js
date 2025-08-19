import { ref } from "vue";
import { defineStore } from "pinia";

import { getDefaultLang, setLang as configSetLang } from "@/api/config/index";

export const LANG_EN = "en";
export const LANG_ZH = "zh";

// 界面语言设置
export const useLangStore = defineStore("lang", () => {
  const lang = ref(getDefaultLang());

  function setLang(value) {
    lang.value = value;

    configSetLang(value);
  }

  return {
    lang,
    setLang,
  };
});
