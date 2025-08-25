<!-- 应用列表 1 项

-->

<template>
  <div class="app-list-item">
    <v-list-item color="primary">
      <template v-slot:prepend>
        <v-avatar rounded="0">
          <v-img :src="i.img" />
        </v-avatar>
      </template>

      <v-list-item-title>{{ i.label }}</v-list-item-title>
      <v-list-item-subtitle>
        <div class="info" @click="toggleShowPath">
          <div class="id">
            <v-badge
              class="sdk"
              inline
              location="top right"
              :content="i.minSdk"
            />
            <span>{{ i.id }}</span>
          </div>
          <div class="version">
            <span>{{ i.versionName }}</span>
            <v-badge
              class="code"
              inline
              location="top right"
              color="primary"
              :content="i.versionCode"
            />
          </div>
        </div>

        <div class="path" v-if="showPath">
          <span>{{ i.apk }}</span>
        </div>
      </v-list-item-subtitle>
    </v-list-item>

    <v-divider inset />
  </div>
</template>

<script setup>
import { computed, ref } from "vue";

const p = defineProps({
  item: Object,
});

// 显示 apk 文件路径
const showPath = ref(false);

const i = computed(() => p.item);

function toggleShowPath() {
  showPath.value = !showPath.value;
}
</script>

<style lang="scss" scoped>
.info {
  display: flex;
  flex-wrap: wrap;
  line-height: 1.4em;

  span {
    word-break: break-all;
  }

  .id {
    flex-shrink: 0;
    max-width: 100%;

    .sdk {
      opacity: 0.6;

      &:deep() .v-badge__wrapper {
        margin-left: 0;
      }
    }
  }

  .version {
    flex-shrink: 0;
    max-width: 100%;

    flex-grow: 1;
    padding-left: 2em;
    text-align: right;

    .code:deep() .v-badge__wrapper {
      margin-right: 0;
    }
  }
}

.path {
  word-break: break-all;
}
</style>
