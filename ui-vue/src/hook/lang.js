// 界面语言
import { computed } from "vue";

import { useLangStore } from "@/stores/lang";
export { LANG_EN, LANG_ZH, PLATFORM_ANDROID, PLATFORM_PC } from "@/stores/lang";

// 获取当前界面语言
export function useLang() {
  const store = useLangStore();

  const lang = computed(() => store.lang);

  return lang;
}

// 获取运行平台
export function usePlatform() {
  const store = useLangStore();

  const platform = computed(() => store.platform);

  return platform;
}
