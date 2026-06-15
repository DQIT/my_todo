<script setup lang="ts">
import { ref } from "vue";
import {
  api,
  REPO_URL,
  DEVELOPER_NAME,
  DEVELOPER_URL,
} from "../api";

const version = "1.0.0";

type CheckState = "idle" | "checking" | "latest" | "available" | "error";
const checkState = ref<CheckState>("idle");
const latestVersion = ref("");
const downloadUrl = ref("");

function open(url: string) {
  api.openUrl(url);
}

async function checkUpdate() {
  if (checkState.value === "checking") return;
  checkState.value = "checking";
  try {
    const info = await api.checkUpdate(version);
    if (info.hasUpdate) {
      latestVersion.value = info.latest;
      downloadUrl.value = info.url;
      checkState.value = "available";
    } else {
      checkState.value = "latest";
    }
  } catch {
    checkState.value = "error";
  }
}
</script>

<template>
  <div class="panel">
    <div class="logo">
      <span class="dot" />
    </div>
    <h1>MyToDo</h1>
    <p class="ver">版本 v{{ version }}</p>
    <p class="desc">一款将待办清单像壁纸一样贴在桌面上的轻量工具，<br />让你在不打断工作流的前提下持续掌握任务。</p>

    <!-- 开发者 & 仓库 -->
    <div class="links">
      <button class="link-row" @click="open(DEVELOPER_URL)">
        <span class="k">开发者</span>
        <span class="v">{{ DEVELOPER_NAME }}</span>
      </button>
      <button class="link-row" @click="open(REPO_URL)">
        <span class="k">GitHub</span>
        <span class="v">{{ REPO_URL.replace("https://", "") }}</span>
      </button>
    </div>

    <!-- 检查更新 -->
    <div class="update">
      <button
        v-if="checkState !== 'available'"
        class="btn"
        :disabled="checkState === 'checking'"
        @click="checkUpdate"
      >
        {{
          checkState === "checking"
            ? "检查中…"
            : checkState === "latest"
            ? "已是最新版本"
            : checkState === "error"
            ? "检查失败，点击重试"
            : "检查更新"
        }}
      </button>
      <button v-else class="btn primary" @click="open(downloadUrl)">
        发现新版本 v{{ latestVersion }} · 前往下载
      </button>
    </div>

    <div class="meta">
      <span>© 2026 MyToDo</span>
      <span>·</span>
      <span>Tauri + Vue 构建</span>
    </div>
  </div>
</template>

<style scoped>
.panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--sp-3);
  text-align: center;
  padding: var(--sp-6);
}
.logo {
  width: 72px;
  height: 72px;
  border-radius: 20px;
  background: linear-gradient(135deg, var(--accent), #8a6bff);
  display: grid;
  place-items: center;
  box-shadow: 0 8px 24px rgba(79, 140, 255, 0.4);
  margin-bottom: var(--sp-2);
}
.dot {
  width: 22px;
  height: 22px;
  border-radius: 7px;
  background: #fff;
}
h1 {
  font-size: 22px;
  font-weight: 600;
}
.ver {
  color: var(--text-secondary);
  font-size: 13px;
}
.desc {
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1.7;
  margin-top: var(--sp-2);
}
/* 开发者 / 仓库链接 */
.links {
  width: 100%;
  max-width: 320px;
  margin-top: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
}
.link-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-btn);
  padding: 10px var(--sp-3);
  cursor: pointer;
  color: var(--text-primary);
  transition: background var(--dur-hover) var(--ease),
    border-color var(--dur-hover) var(--ease);
}
.link-row:hover {
  background: var(--bg-card-hover);
  border-color: var(--accent);
}
.link-row .k {
  font-size: 13px;
  color: var(--text-secondary);
}
.link-row .v {
  font-size: 13px;
  color: var(--accent);
  max-width: 200px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
/* 检查更新 */
.update {
  width: 100%;
  max-width: 320px;
  margin-top: var(--sp-3);
}
.btn {
  width: 100%;
  padding: 10px var(--sp-3);
  font-size: 13px;
  border-radius: var(--radius-btn);
  border: 1px solid var(--border);
  background: var(--bg-card);
  color: var(--text-primary);
  cursor: pointer;
  transition: background var(--dur-hover) var(--ease),
    border-color var(--dur-hover) var(--ease);
}
.btn:hover:not(:disabled) {
  background: var(--bg-card-hover);
  border-color: var(--accent);
}
.btn:disabled {
  opacity: 0.6;
  cursor: default;
}
.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}
.btn.primary:hover {
  filter: brightness(1.08);
}
.meta {
  display: flex;
  gap: var(--sp-2);
  color: var(--text-disabled);
  font-size: 12px;
  margin-top: var(--sp-4);
}
</style>
