<!-- 列表空白占位 (无数据)

-->

<template>
  <div class="list-empty">
    <span>
      <slot>{{ t.c.no_data }}</slot>
    </span>
  </div>
</template>

<script setup>
import { useT } from "@/i18n/index";

const t = useT();
</script>

<style lang="scss" scoped>
.list-empty {
  padding: 1em;
  text-align: center;

  color: #777;
}
</style>
