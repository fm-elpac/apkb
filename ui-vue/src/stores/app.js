// 本机应用 (apk) 备份功能
import { computed, ref } from "vue";
import { defineStore } from "pinia";

import {
  debug_log,
  debug_write_cache,
  pm_get_apk_list,
  pm_get_apk_list_start,
} from "@/api/apkb_api";

import { PLATFORM_ANDROID, useLangStore } from "./lang";

export const useAppStore = defineStore("app", () => {
  // 本机安装的 apk 列表
  const host_list = ref([]);
  // 加载中 状态
  const host_list_loading = ref(false);

  // 用户选择备份的 apk id 列表 (保存到 localStorage)
  const app_id_list = ref([]);

  // 用户选择的应用信息 (可从 host_list/app_id_list 计算出来, 非实时更新)
  const app_list = ref([]);

  // TODO
  // 备份列表 (目前只有 1 个位置: "内置存储" AZI_DIR_SDCARD_DATA/backup/)
  const backup_list = ref([]);

  // 本机应用列表 (显示)
  const hostListShow = computed(() => {
    const lang = useLangStore();

    const o = Array.from(host_list.value);

    // android
    if (PLATFORM_ANDROID == lang.platform) {
      for (const i of o) {
        i.img = "file://" + i.icon;
      }
    }
    // 按名称排序
    o.sort((a, b) => (a.label > b.label) ? 1 : (a.label < b.label) ? -1 : 0);

    return o;
  });

  function setAppIdList(list) {
    // TODO
    // TODO 保存到 localStorage
  }

  // 重新计算 app_list
  function updateAppList() {
    // TODO
  }

  // 加载 host_list 拉取结果
  function checkResult() {
    const r = pm_get_apk_list();
    if (null != r) {
      host_list.value = r.list;

      if (r.done) {
        host_list_loading.value = false;
        updateAppList();

        // DEBUG
        debug_write_cache(
          "host_list.json",
          JSON.stringify(host_list.value, "", "  "),
        );
      } else {
        setTimeout(checkResult, 500);
      }
    }
    // DEBUG
    host.value = [
      {
        "id": "com.xiaomi.scanner",
        "versionCode": 1322032440,
        "versionName": "15.9.23",
        "label": "AI扫描",
        "icon":
          "/storage/emulated/0/Android/data/io.github.fm_elpac.apkb/cache/icon/com.xiaomi.scanner.png",
        "minSdk": 24,
        "apk":
          "/data/app/~~Rih9hQ_iG3CKj-sN3QNpqA==/com.xiaomi.scanner-RJq-23DrrBIZaNi9WDbuxQ==/base.apk",
      },
    ];
  }

  // 开始加载 host_list (后台新线程异步进行)
  function loadHostList() {
    host_list_loading.value = true;
    pm_get_apk_list_start();

    checkResult();
  }

  return {
    host_list,
    host_list_loading,
    hostListShow,

    app_id_list,
    app_list,
    backup_list,

    setAppIdList,
    loadHostList,
  };
});
