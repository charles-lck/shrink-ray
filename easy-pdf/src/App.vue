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
import { getCurrentWindow } from '@tauri-apps/api/window';

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
themeMode.value = appStore.themeMode === 'light' ? null : darkTheme



// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();

document
  .getElementById('titlebar-minimize')
  ?.addEventListener('click', () => appWindow.minimize());
document
  .getElementById('titlebar-maximize')
  ?.addEventListener('click', () => appWindow.toggleMaximize());
document
  .getElementById('titlebar-close')
  ?.addEventListener('click', () => appWindow.close());
</script>