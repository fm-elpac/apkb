<!-- 本机 页面: 显示本机已安装的应用 (apk) 列表, 供选择

-->

<template>
  <div class="page-host">
    <div v-if="loading" class="loading-box">
      <v-progress-circular color="primary" indeterminate />
    </div>
    <!--
    <div class="todo">
      <pre>{{ JSON.stringify(list, '', '  ') }}</pre>
    </div>
    -->

    <v-list lines="two">
      <template v-for="i in showList" :key="i.id">
        <v-list-item color="primary">
          <template v-slot:prepend>
            <v-avatar rounded="0">
              <v-img :src="i.img" />
            </v-avatar>
          </template>

          <v-list-item-title>{{ i.label }}</v-list-item-title>
          <v-list-item-subtitle>
            <div class="info">
              <div class="id">
                <v-badge
                  class="sdk"
                  inline
                  location="top right"
                  :content="i.minSdk"
                />
                <span>{{ i.id }}</span>
              </div>
              <div class="version">
                <span>{{ i.versionName }}</span>
                <v-badge
                  class="code"
                  inline
                  location="top right"
                  color="primary"
                  :content="i.versionCode"
                />
              </div>
            </div>
          </v-list-item-subtitle>
        </v-list-item>

        <v-divider inset />
      </template>
    </v-list>

    <ListEmpty v-if="list.length < 1" />
    <ListEmpty v-else>{{ t.host.total_app(list.length) }}</ListEmpty>
  </div>
</template>

<script setup>
import { computed, onMounted, ref } from "vue";

import { pm_get_apk_list, pm_get_apk_list_start } from "@/api/apkb_api";

import ListEmpty from "@/c/ListEmpty.vue";

import { useT } from "@/i18n/index";

const t = useT();

const list = ref([]);
const loading = ref(false);

const showList = computed(() => {
  const o = Array.from(list.value);
  for (const i of o) {
    i.img = "file://" + i.icon;
  }
  // 按名称排序
  o.sort((a, b) => (a.label > b.label) ? 1 : (a.label < b.label) ? -1 : 0);

  return o;
});

function checkResult() {
  const r = pm_get_apk_list();
  if (null != r) {
    list.value = r.list;

    if (r.done) {
      loading.value = false;
    } else {
      setTimeout(checkResult, 500);
    }
  }
}

onMounted(() => {
  loading.value = true;
  pm_get_apk_list_start();

  checkResult();
});
</script>

<style lang="scss" scoped>
.loading-box {
  padding: 1em;
  display: flex;
  justify-content: center;
}
// .todo {
//   padding: 1em;
//   overflow: auto;
// }

.info {
  display: flex;
  flex-wrap: wrap;
  line-height: 1.4em;

  span {
    word-break: break-all;
  }

  .id {
    flex-shrink: 0;
    max-width: 100%;

    .sdk {
      opacity: 0.6;

      &:deep() .v-badge__wrapper {
        margin-left: 0;
      }
    }
  }

  .version {
    flex-shrink: 0;
    max-width: 100%;

    flex-grow: 1;
    padding-left: 2em;
    text-align: right;

    .code:deep() .v-badge__wrapper {
      margin-right: 0;
    }
  }
}
</style>
