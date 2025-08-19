// 界面语言
import { computed } from "vue";

import { useLangStore } from "@/stores/lang";

// 获取当前界面语言
export function useLang() {
  const store = useLangStore();

  const lang = computed(() => store.lang);

  return lang;
}
