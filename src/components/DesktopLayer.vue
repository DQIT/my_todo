<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { loadAll, settings, pendingTodos } from "../store";
import { api } from "../api";
import { isExpired, formatTime } from "../utils";
import type { Todo } from "../types";

const items = ref<Todo[]>([]);
const containerRef = ref<HTMLElement | null>(null);
const visibleCount = ref<number>(999);

let unlisten: (() => void) | null = null;
let ro: ResizeObserver | null = null;

function refresh() {
  items.value = pendingTodos();
  requestAnimationFrame(measure);
}

const unlocked = computed(() => !settings.desktopLocked);
const isLightText = computed(() => settings.textColor === "light");

// 面板背景：透明度 0-100 → alpha。0 时纯浮动文字（贴合壁纸概念）。
const panelStyle = computed(() => {
  const a = settings.backgroundOpacity / 100;
  const base = isLightText.value ? "20, 22, 28" : "248, 249, 252";
  return { background: `rgba(${base}, ${a})` };
});

const textStyle = computed(() => {
  const color = isLightText.value ? "#ffffff" : "#16161a";
  const shadow = settings.textShadow
    ? isLightText.value
      ? "0 1px 3px rgba(0,0,0,0.7)"
      : "0 1px 2px rgba(255,255,255,0.7)"
    : "none";
  return {
    color,
    fontSize: `${settings.fontSize}px`,
    textShadow: shadow,
  };
});

const timeStyle = computed(() => ({
  fontSize: `${Math.max(11, settings.fontSize - 4)}px`,
}));

// 自适应：高度不足时优先显示时间更近的（已升序），整条裁剪超出部分。
function measure() {
  const el = containerRef.value;
  if (!el) return;
  const max = el.clientHeight;
  const children = Array.from(el.querySelectorAll<HTMLElement>(".item"));
  let count = 0;
  for (const child of children) {
    if (child.offsetTop + child.offsetHeight > max + 0.5) break;
    count++;
  }
  visibleCount.value = count;
}

onMounted(async () => {
  await loadAll();
  refresh();
  unlisten = await api.onStateChanged(async () => {
    await loadAll();
    refresh();
  });
  if (containerRef.value) {
    ro = new ResizeObserver(() => measure());
    ro.observe(containerRef.value);
  }
  window.addEventListener("resize", measure);
});

onUnmounted(() => {
  unlisten?.();
  ro?.disconnect();
  window.removeEventListener("resize", measure);
});
</script>

<template>
  <!-- 外层留透明 gutter 供窗口边缘缩放（解锁态） -->
  <div class="layer-root" :class="{ unlocked }">
    <!-- 内容面板：解锁态整块可拖拽移动 -->
    <div
      class="panel"
      :class="{ unlocked }"
      :style="panelStyle"
      :data-tauri-drag-region="unlocked ? '' : null"
    >
      <!-- 解锁态顶部提示条 -->
      <div v-if="unlocked" class="hint" :data-tauri-drag-region="''">
        <span class="grip">⠿</span> 拖拽移动 · 边缘缩放
      </div>

      <div ref="containerRef" class="rows">
        <div
          v-for="(t, i) in items"
          :key="t.id"
          class="item"
          :class="{
            hidden: i >= visibleCount,
            expired: settings.expiredHighlight && isExpired(t),
          }"
          :data-tauri-drag-region="unlocked ? '' : null"
        >
          <span class="bar" />
          <div class="texts">
            <div
              v-if="settings.displayFormat === 'time-content'"
              class="time"
              :style="[textStyle, timeStyle]"
            >
              {{ formatTime(t.time) }}
            </div>
            <div class="content" :style="textStyle">{{ t.content }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.layer-root {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  padding: 0;
}
/* 解锁态留出 8px 边缘缩放区 */
.layer-root.unlocked {
  padding: 8px;
}
.panel {
  width: 100%;
  height: 100%;
  border-radius: 14px;
  padding: 14px 16px;
  overflow: hidden;
  border: 1px solid transparent;
  transition: border-color var(--dur-switch) var(--ease),
    box-shadow var(--dur-switch) var(--ease);
}
.panel.unlocked {
  border-color: rgba(79, 140, 255, 0.55);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.28);
}
.hint {
  display: flex;
  align-items: center;
  gap: 6px;
  justify-content: center;
  font-size: 11px;
  color: rgba(255, 255, 255, 0.85);
  background: rgba(79, 140, 255, 0.55);
  padding: 3px 0;
  margin: -6px -8px 10px;
  border-radius: 8px;
  cursor: move;
  user-select: none;
}
.grip {
  letter-spacing: -2px;
}
.rows {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
  overflow: hidden;
}
.item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  min-width: 0;
}
.item.hidden {
  visibility: hidden;
}
/* 左侧强调色竖条，提供容器/层次感 */
.bar {
  flex-shrink: 0;
  width: 3px;
  align-self: stretch;
  min-height: 1.2em;
  margin-top: 2px;
  border-radius: 2px;
  background: var(--accent);
  opacity: 0.9;
}
.item.expired .bar {
  background: #ff8585;
}
.texts {
  min-width: 0;
  flex: 1;
}
.time {
  opacity: 0.72;
  line-height: 1.3;
  margin-bottom: 1px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.content {
  font-weight: 500;
  line-height: 1.35;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.item.expired .content,
.item.expired .time {
  color: #ff8585 !important;
}
</style>
