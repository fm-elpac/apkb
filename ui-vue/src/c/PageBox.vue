<!-- 内容页面 (带样式)

-->

<template>
  <div class="page-box">
    <slot />
  </div>
</template>

<script setup>
</script>

<style lang="scss" scoped>
.page-box {
  margin: 0.8em;
}

:deep() {
  h2 {
    margin-bottom: 0.2em;
  }

  p {
    margin-bottom: 0.6em;
  }

  code {
    background-color: white;
    word-break: break-all;
  }

  .v-card code {
    background-color: rgba(0, 0, 0, 0.1);
  }

  h3 {
    margin-top: 1em;
    margin-bottom: 0.4em;
  }

  pre {
    padding: 0.6em;
    font-size: 0.8em;
    background-color: rgba(0, 0, 0, 0.1);
    overflow-x: auto;
  }

  ul {
    padding-left: 1.2em;
  }

  hr {
    margin: 1em 0;
  }

  img {
    max-width: 100%;
    height: auto;
    max-height: 700px;
    display: block;
    margin: 0 auto 1em auto;
  }
}
</style>
