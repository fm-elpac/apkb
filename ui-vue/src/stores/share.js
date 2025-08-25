// 分享功能 (mDNS/DNS-SD) (NsdManager Android)
import { ref } from "vue";
import { defineStore } from "pinia";

import { useLangStore } from "./lang";

export const useShareStore = defineStore("share", () => {
  // 设备名称 (用于 mDNS/DNS-SD, 用户可修改) 保存到 localStorage
  const device_name = ref("");

  // TODO

  return {
    device_name,
  };
});
