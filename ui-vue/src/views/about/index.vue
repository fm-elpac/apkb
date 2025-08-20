<!-- 关于 页面

-->

<template>
  <PageBox class="page-about">
    <Zh v-if="LANG_ZH == lang" />
    <En v-else />

    <!-- 共同部分 -->
    <p>
      <ul>
        <li><code>TODO</code></li>
      </ul>
    </p>

    <h3>LICENSE</h3>
    <pre>
apkb: Backup your Android app and share
Copyright (C) 2025  secext2022

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see &lt;https://www.gnu.org/licenses/&gt;.</pre>
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
