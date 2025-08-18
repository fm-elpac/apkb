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
import { inject, ref } from "vue";
import { useRouter } from "vue-router";

const showSidebar = inject("showSidebar");

// 导航菜单
const list = ref([
  {
    title: "App",
    value: "/ui/home",
  },
]);

const router = useRouter();

function select(data) {
  const path = data.id;
  router.push(path);
}
</script>

<style lang="scss" scoped>
</style>
