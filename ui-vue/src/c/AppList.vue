<!-- 应用 (信息) 列表 封装

-->

<template>
  <div class="app-list">
    <LoadingTop v-if="p.loading" />

    <v-list lines="two">
      <AppListItem
        v-for="i in p.list"
        :key="i.id"
        :item="i"
      />
    </v-list>

    <TotalApp :len="p.list.length" />
  </div>
</template>

<script setup>
import LoadingTop from "./LoadingTop.vue";
import AppListItem from "./AppListItem.vue";
import TotalApp from "./TotalApp.vue";

const p = defineProps({
  list: Array,
  loading: Boolean,
});
</script>

<style lang="scss" scoped>
</style>
