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

import {
  mdiAndroid,
  mdiBackupRestore,
  mdiCog,
  mdiCreationOutline,
  mdiInformationOutline,
  mdiShareVariantOutline,
  mdiTabletCellphone,
} from "@mdi/js";

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
      props: {
        prependIcon: mdiCreationOutline,
      },
    },
    {
      title: i.app,
      value: "/ui/app",
      props: {
        prependIcon: mdiAndroid,
      },
    },
    {
      title: i.backup,
      value: "/ui/backup",
      props: {
        prependIcon: mdiBackupRestore,
      },
    },
    {
      title: i.host,
      value: "/ui/host",
      props: {
        prependIcon: mdiTabletCellphone,
      },
    },
    {
      title: i.share,
      value: "/ui/share",
      props: {
        prependIcon: mdiShareVariantOutline,
      },
    },
    {
      title: i.setting,
      value: "/ui/setting",
      props: {
        prependIcon: mdiCog,
      },
    },
    {
      title: i.about,
      value: "/ui/about",
      props: {
        prependIcon: mdiInformationOutline,
      },
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
