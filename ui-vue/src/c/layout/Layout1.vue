<!-- 页面布局 (1)

-->

<template>
  <v-layout class="layout-1">
    <v-app-bar :elevation="2" color="primary">
      <template #prepend>
        <v-app-bar-nav-icon @click.stop="toggleSidebar" />
      </template>

      <v-app-bar-title>{{ title }}</v-app-bar-title>
    </v-app-bar>

    <Sidebar />

    <v-main>
      <slot />
    </v-main>
  </v-layout>
</template>

<script setup>
import { computed, provide, ref } from "vue";
import { useRoute } from "vue-router";

import { useT } from "@/i18n/index";

import Sidebar from "./Sidebar.vue";

const route = useRoute();

const showSidebar = ref(null);
provide("showSidebar", showSidebar);

const t = useT();

const title = computed(() => t.value.title[route.meta.title]);

function toggleSidebar() {
  showSidebar.value = !showSidebar.value;
}
</script>

<style lang="scss" scoped>
</style>
