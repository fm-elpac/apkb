<!-- 列表顶部 加载中 指示组件

-->

<template>
  <div class="loading-top">
    <v-progress-circular color="primary" indeterminate />
  </div>
</template>

<script setup>
</script>

<style lang="scss" scoped>
.loading-top {
  padding: 1em;
  display: flex;
  justify-content: center;
}
</style>
