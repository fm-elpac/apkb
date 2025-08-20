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
  globalThis.setPlatform = langStore.setPlatform;

  console.log("lang =", langStore.lang);
  console.log("platform =", langStore.platform);
});
</script>

<style lang="scss" scoped>
</style>
