<template>
  <div id="app">
    <Suspense>
      <n-config-provider :theme="themeMode">
        <router-view></router-view>
      </n-config-provider>
    </Suspense>
  </div>
</template>


<script setup lang="ts">
import { ref, watch } from 'vue';
import { NConfigProvider } from 'naive-ui'
import { createTheme, selectDark } from 'naive-ui'
import { useI18n } from 'vue-i18n';
import { useAppStore } from "./store";

const darkTheme = createTheme([selectDark])
let themeMode: any = ref(darkTheme)
const { locale } = useI18n();
const appStore = useAppStore();
watch(() => appStore.language, (newValue: string) => {
  locale.value = newValue;
})
watch(() => appStore.themeMode, (newValue: string) => {
  themeMode.value = newValue === 'light' ? null : darkTheme
})
locale.value = appStore.language;
</script>