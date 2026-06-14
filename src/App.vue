<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { api } from "./api";
import { loadAll, settings, applyTheme } from "./store";
import TasksPanel from "./components/TasksPanel.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import AboutPanel from "./components/AboutPanel.vue";
import TitleBar from "./components/TitleBar.vue";

type Tab = "tasks" | "settings" | "about";
const tab = ref<Tab>("tasks");
const tabs: { key: Tab; label: string }[] = [
  { key: "tasks", label: "事项管理" },
  { key: "settings", label: "软件设置" },
  { key: "about", label: "关于我们" },
];

let unlisten: (() => void) | null = null;
let mql: MediaQueryList | null = null;

onMounted(async () => {
  await loadAll();
  // 后端/另一窗口状态变更 → 重新加载
  unlisten = await api.onStateChanged(async () => {
    await loadAll();
  });
  // 跟随系统主题
  mql = window.matchMedia("(prefers-color-scheme: light)");
  mql.addEventListener("change", () => {
    if (settings.theme === "system") applyTheme();
  });
});

onUnmounted(() => {
  unlisten?.();
});
</script>

<template>
  <div class="app-shell">
    <TitleBar />
    <nav class="tabbar">
      <button
        v-for="t in tabs"
        :key="t.key"
        class="tab"
        :class="{ active: tab === t.key }"
        @click="tab = t.key"
      >
        {{ t.label }}
        <span class="indicator" v-if="tab === t.key" />
      </button>
    </nav>
    <main class="content">
      <Transition name="fade" mode="out-in">
        <TasksPanel v-if="tab === 'tasks'" key="tasks" />
        <SettingsPanel v-else-if="tab === 'settings'" key="settings" />
        <AboutPanel v-else key="about" />
      </Transition>
    </main>
  </div>
</template>

<style scoped>
.tabbar {
  display: flex;
  gap: var(--sp-1);
  padding: 0 var(--sp-4);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}
.tab {
  position: relative;
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 14px;
  padding: var(--sp-3) var(--sp-3);
  cursor: pointer;
  transition: color var(--dur-hover) var(--ease);
}
.tab:hover {
  color: var(--text-primary);
}
.tab.active {
  color: var(--text-primary);
  font-weight: 500;
}
.indicator {
  position: absolute;
  left: var(--sp-3);
  right: var(--sp-3);
  bottom: -1px;
  height: 2px;
  background: var(--accent);
  border-radius: 2px;
}
.content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--dur-switch) var(--ease),
    transform var(--dur-switch) var(--ease);
}
.fade-enter-from {
  opacity: 0;
  transform: translateY(6px);
}
.fade-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}
</style>
