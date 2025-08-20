<!-- 欢迎 页面

-->

<template>
  <PageBox class="page-welcome">
    <En v-if="LANG_EN == lang" />
    <Zh v-if="LANG_ZH == lang" />
  </PageBox>
</template>

<script setup>
import PageBox from "@/c/PageBox.vue";

import { LANG_EN, LANG_ZH, useLang } from "@/hook/lang";

import En from "./En.vue";
import Zh from "./Zh.vue";

const lang = useLang();
</script>

<style lang="scss" scoped>
</style>
