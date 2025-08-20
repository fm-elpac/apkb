<!-- 页面布局 (1) 用于 router

-->

<template>
  <Layout1 class="layout-r1">
    <RouterView />
  </Layout1>
</template>

<script setup>
import { RouterView } from "vue-router";

import Layout1 from "./Layout1.vue";
</script>

<style lang="scss" scoped>
</style>
