<script setup lang="ts">
import { api } from "../api";

async function minimize() {
  if (!api.isTauri()) return;
  const { getCurrentWindow } = await import("@tauri-apps/api/window");
  await getCurrentWindow().minimize();
}

// 关闭 = 隐藏到托盘（不退出）
async function closeToTray() {
  if (!api.isTauri()) {
    console.info("[预览] 关闭→隐藏到托盘");
    return;
  }
  await api.hideMainWindow();
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <span class="logo">
        <span class="logo-dot" />
      </span>
      <span class="name">MyToDo</span>
    </div>
    <div class="actions">
      <button class="winbtn" title="最小化" @click="minimize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="2" y="5.5" width="8" height="1" fill="currentColor" />
        </svg>
      </button>
      <button class="winbtn close" title="隐藏到托盘" @click="closeToTray">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            d="M2 2 L10 10 M10 2 L2 10"
            stroke="currentColor"
            stroke-width="1.2"
            fill="none"
          />
        </svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  height: 44px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--sp-2) 0 var(--sp-4);
  flex-shrink: 0;
  -webkit-app-region: drag;
}
.brand {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}
.logo {
  width: 22px;
  height: 22px;
  border-radius: 7px;
  background: var(--accent-grad);
  display: grid;
  place-items: center;
  box-shadow: 0 4px 12px rgba(79, 140, 255, 0.4);
}
.logo-dot {
  width: 8px;
  height: 8px;
  border-radius: 3px;
  background: #fff;
}
.name {
  font-weight: 600;
  font-size: 13px;
  letter-spacing: 0.3px;
}
.actions {
  display: flex;
  gap: 2px;
  -webkit-app-region: no-drag;
}
.winbtn {
  width: 32px;
  height: 28px;
  display: grid;
  place-items: center;
  background: none;
  border: none;
  border-radius: var(--radius-btn);
  color: var(--text-secondary);
  cursor: pointer;
  transition: background var(--dur-hover) var(--ease),
    color var(--dur-hover) var(--ease);
}
.winbtn:hover {
  background: var(--bg-card-hover);
  color: var(--text-primary);
}
.winbtn.close:hover {
  background: var(--danger);
  color: #fff;
}
</style>
