<!-- 设置 页面

-->

<template>
  <PageBox class="page-setting">
    <!-- 设置界面语言 -->
    <p>
      <v-select
        :label="langLabel"
        variant="solo"
        :items="langList"
        :model-value="langStore.lang"
        @update:model-value="langStore.setLang"
      />
    </p>

    <!-- TODO -->
  </PageBox>
</template>

<script setup>
import { computed, ref } from "vue";

import PageBox from "@/c/PageBox.vue";

import { LANG_EN, LANG_ZH, useLangStore } from "@/stores/lang";

const langStore = useLangStore();

const langLabel = computed(() =>
  (LANG_EN == langStore.lang) ? "Language (语言)" : "界面语言 (Language)"
);

// 可用语言列表
const langList = ref([
  {
    title: "中文 (Chinese)",
    value: LANG_ZH,
  },
  {
    title: "English (英文)",
    value: LANG_EN,
  },
]);
</script>

<style lang="scss" scoped>
</style>
