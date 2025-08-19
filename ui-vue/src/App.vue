<!--

-->

<template>
  <RouterView />
</template>

<script setup>
import { onMounted } from "vue";
import { RouterView } from "vue-router";

import { useLangStore } from "@/stores/lang";

const langStore = useLangStore();

onMounted(() => {
  // DEBUG
  globalThis.setLang = langStore.setLang;

  console.log("lang =", langStore.lang);
});
</script>

<style lang="scss" scoped>
</style>
