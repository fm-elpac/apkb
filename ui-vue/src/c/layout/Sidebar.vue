<!-- 侧边栏 (主菜单)

-->

<template>
  <v-navigation-drawer
    class="sidebar"
    v-model="showSidebar"
  >
    <v-list
      :items="list"
      @click:select="select"
    />
  </v-navigation-drawer>
</template>

<script setup>
import { computed, inject } from "vue";
import { useRouter } from "vue-router";

const showSidebar = inject("showSidebar");

import { useT } from "@/i18n/index";

const t = useT();

// 导航菜单
const list = computed(() => {
  const i = t.value.title;

  return [
    {
      title: i.welcome,
      value: "/ui/welcome",
    },
    {
      title: i.app,
      value: "/ui/app",
    },
    {
      title: i.backup,
      value: "/ui/backup",
    },
    {
      title: i.host,
      value: "/ui/host",
    },
    {
      title: i.share,
      value: "/ui/share",
    },
    {
      title: i.setting,
      value: "/ui/setting",
    },
    {
      title: i.about,
      value: "/ui/about",
    },
  ];
});

const router = useRouter();

function select(data) {
  const path = data.id;
  router.push(path);
}
</script>

<style lang="scss" scoped>
</style>
