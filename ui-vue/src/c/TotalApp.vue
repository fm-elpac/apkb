<!-- 列表底部 无数据/共 n 个应用

-->

<template>
  <div class="total-app">
    <ListEmpty v-if="p.len < 1" />
    <ListEmpty v-else>{{ t.c.total_app(p.len) }}</ListEmpty>
  </div>
</template>

<script setup>
import { useT } from "@/i18n/index";

import ListEmpty from "./ListEmpty.vue";

const p = defineProps({
  len: Number,
});

const t = useT();
</script>

<style lang="scss" scoped>
</style>
