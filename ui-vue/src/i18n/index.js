// 多语言翻译
import { computed } from "vue";

import { LANG_EN, LANG_ZH } from "@/stores/lang";
import { useLang } from "@/hook/lang";

import en from "./en/index";
import zh from "./zh/index";

// 翻译数据
export function useT() {
  const lang = useLang();

  const t = computed(() => {
    if (LANG_ZH == lang.value) {
      return zh;
    }
    if (LANG_EN == lang.value) {
      return en;
    }
  });

  return t;
}
