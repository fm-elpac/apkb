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
      <RouterView />
    </v-main>
  </v-layout>
</template>

<script setup>
import { computed, provide, ref } from "vue";
import { RouterView, useRoute } from "vue-router";

import Sidebar from "./Sidebar.vue";

const route = useRoute();

const showSidebar = ref(null);
provide("showSidebar", showSidebar);

const title = computed(() => route.meta.title);

function toggleSidebar() {
  showSidebar.value = !showSidebar.value;
}
</script>

<style lang="scss" scoped>
</style>
