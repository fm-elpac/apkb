<!-- 本机 页面: 显示本机已安装的应用 (apk) 列表, 供选择

-->

<template>
  <div class="page-host">
    <AppList :list="app.hostListShow" :loading="app.host_list_loading" />
  </div>
</template>

<script setup>
import { onMounted, watch } from "vue";

import { useAppStore } from "@/stores/app";

import AppList from "@/c/AppList.vue";

const app = useAppStore();

onMounted(() => {
  app.loadHostList();
});
</script>

<style lang="scss" scoped>
</style>
