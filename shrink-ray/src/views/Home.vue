<template>
  <div id="app" :class="['app-container', { 'dark-theme': appStore.themeMode === 'dark' }]">
    <h1 class="title">👋 {{ $t('welcome') }}</h1>

    <!-- 主题切换按钮 -->
    <div class="theme-toggle">
      <button @click="toggleTheme" class="theme-btn">
        {{ appStore.themeMode === "light" ? $t('theme.toggleToDark') : $t('theme.toggleToLight') }}
      </button>
      <n-select class="lang-btn" v-model:value="appStore.language" :options="languageOptions" />
    </div>

    <div class="main-card">
      <!-- 压缩选项 -->
      <div class="options-container">
        <!-- 压缩模式 -->
        <div class="option-item">
          <label class="option-label">🗜️ {{ $t('compressMode') }}:</label>
          <div class="checkbox-wrapper">
            <input type="checkbox" v-model="lossless" :disabled="isCompressing" class="checkbox" />
            <span class="checkbox-text">
              {{ lossless ? `${$t('lossless')} (oxipng)` : `${$t('lossy')} (pngquant)` }}
            </span>
          </div>
        </div>

        <!-- 替换原文件 -->
        <div class="option-item">
          <label class="option-label">{{ $t('replace') }}:</label>
          <div class="checkbox-wrapper">
            <input type="checkbox" v-model="replace" :disabled="isCompressing" class="checkbox" />
          </div>
        </div>
      </div>

      <!-- 拖放区域 -->
      <div class="drop-area">
        <p class="drop-text" v-html="$t('dropText')"></p>
        <p class="drop-hint">{{ $t('dropHint') }}</p>
      </div>

      <!-- 任务列表 -->
      <div v-if="tasks.length > 0" class="task-list">
        <div v-for="task in tasks" :key="task.id" class="task-item">
          <div class="task-header">
            <p class="task-name">{{ task.name }} - {{ $t(`taskStatus.${task.status}`) }}</p>
            <button v-if="task.status === 'completed' && !replace" class="download-btn"
              @click="downloadSingle(task.outputPath)">
              {{ $t('download') }}
            </button>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: task.progress + '%' }"></div>
          </div>
          <p class="progress-text">{{ $t('progress') }}: {{ task.progress }}%</p>
        </div>

        <!-- 打包下载按钮 -->
        <button v-if="allCompleted && !replace" class="zip-btn" @click="downloadZip">
          {{ $t('zipDownload') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onUnmounted, computed, watch } from "vue";
import { NSelect } from "naive-ui";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile, readFile } from "@tauri-apps/plugin-fs";
import JSZip from "jszip";
import { useAppStore } from "../store";

const appStore = useAppStore();
const languageOptions = [
  {
    label: 'English', // 英文
    value: 'en'
  },
  {
    label: '简体中文', // 中文
    value: 'zh_CN'
  },
  {
    label: '繁體中文', // 中文
    value: 'zh_TW'
  },
  {
    label: '日本語', // 日文
    value: 'ja'
  },
  {
    label: '한국어', // 韩文
    value: 'ko'
  },
  {
    label: 'Español', // 西班牙文
    value: 'es'
  },
  {
    label: 'Français', // 法文
    value: 'fr'
  }
];

// 切换主题
const toggleTheme = () => {
  appStore.themeMode = appStore.themeMode === "light" ? "dark" : "light";
};

// 压缩相关逻辑
const lossless = ref(false);
const replace = ref(false);
const isCompressing = ref(false);
const tasks = ref([]);
const MAX_FILES = 30;

const allCompleted = computed(() => tasks.value.every((task) => task.status === "completed"));

// 监听 Tauri 的文件拖放事件
const unlistenDrop = listen("tauri://drag-drop", (event) => {
  const files = event.payload.paths;

  if (!Array.isArray(files)) {
    alert("拖放失败：无法识别文件列表");
    return;
  }

  const pngFiles = files.filter((path) => path.toLowerCase().endsWith(".png") || path.toLowerCase().endsWith(".jpg") || path.toLowerCase().endsWith(".jpeg"));
  if (pngFiles.length + tasks.value.length > MAX_FILES) {
    alert(t('dropText') + `，已截取前 ${MAX_FILES - tasks.value.length} 张`);
    pngFiles.splice(MAX_FILES - tasks.value.length);
  }

  pngFiles.forEach((path) => {
    const task = {
      id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      name: path.split(/[\\/]/).pop(),
      path: path,
      progress: 0,
      status: "waiting",
      outputPath: "",
    };
    tasks.value.push(task);
  });

  if (pngFiles.length > 0) {
    startCompression();
  }
});

// 监听进度事件
const unlistenProgress = listen("progress-update", (event) => {
  const payload = event.payload;
  const task = tasks.value.find((t) => t.id === payload.task_id);
  if (task) {
    task.progress = payload.progress;
    if (payload.progress === 100) {
      task.status = "completed";
    }
  }
});

// 开始压缩
const startCompression = async () => {
  if (isCompressing.value) return;
  isCompressing.value = true;

  const processNext = async () => {
    const nextTask = tasks.value.find((t) => t.status === "waiting");
    if (!nextTask) {
      isCompressing.value = false;
      return;
    }

    nextTask.status = "compressing";
    try {
      const outputPath = await invoke("compress_image", {
        taskId: nextTask.id,
        path: nextTask.path,
        lossless: lossless.value,
        replace: replace.value,
      });
      nextTask.outputPath = outputPath;
    } catch (error) {
      nextTask.status = "failed: " + error;
    } finally {
      processNext();
    }
  };

  for (let i = 0; i < 5; i++) {
    processNext();
  }
};

// 单个文件下载
const downloadSingle = async (path) => {
  try {
    const data = await readFile(path);
    const ext = path.split(".").pop().toLowerCase();
    const originalFileName = path.split(/[\\/]/).pop();
    const cleanedFileName = originalFileName
      .replace(/@@.*?@@/g, "")
      .replace(/\.\./g, ".");

    const savePath = await save({
      filters: [{ name: ext, extensions: [ext] }],
      defaultPath: cleanedFileName,
    });

    if (!savePath) return;

    const contents = data instanceof Uint8Array ? data : new Uint8Array(data);
    await writeFile(savePath, contents);
  } catch (error) {
    alert("下载失败: " + (error.message || error));
  }
};

// 打包下载
const downloadZip = async () => {
  try {
    const zip = new JSZip();
    for (const task of tasks.value) {
      const data = await readFile(task.outputPath);
      const fileContents = data instanceof Uint8Array ? data : new Uint8Array(data);
      const originalFileName = task.outputPath.split(/[\\/]/).pop();
      const cleanedFileName = originalFileName
        .replace(/@@.*?@@/g, "")
        .replace(/\.\./g, ".");
      zip.file(cleanedFileName, fileContents);
    }

    const content = await zip.generateAsync({ type: "uint8array" });
    const savePath = await save({
      filters: [{ name: "ZIP", extensions: ["zip"] }],
      defaultPath: "compressed_images.zip",
    });

    if (!savePath) return;

    await writeFile(savePath, content);
  } catch (error) {
    alert("打包下载失败: " + (error.message || error));
  }
};

// 清理事件监听
onUnmounted(() => {
  unlistenDrop.then((fn) => fn());
  unlistenProgress.then((fn) => fn());
});
</script>

<style lang="less" scoped>
@import "../assets/css/style.less";
</style>