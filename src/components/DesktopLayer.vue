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
  // 下一帧重算可见条数
  requestAnimationFrame(measure);
}

// 背景色：透明度 0-100 → rgba alpha
const bgStyle = computed(() => {
  const a = settings.backgroundOpacity / 100;
  const isLight = settings.textColor === "dark"; // 文字深色通常配浅背景
  const base = isLight ? "250, 250, 252" : "20, 20, 24";
  return { background: `rgba(${base}, ${a})` };
});

const textStyle = computed(() => {
  const color = settings.textColor === "light" ? "#ffffff" : "#16161a";
  const shadow = settings.textShadow
    ? settings.textColor === "light"
      ? "0 1px 3px rgba(0,0,0,0.65)"
      : "0 1px 2px rgba(255,255,255,0.6)"
    : "none";
  return {
    color,
    fontSize: `${settings.fontSize}px`,
    textShadow: shadow,
  };
});

function lineText(t: Todo): string {
  if (settings.displayFormat === "content") return t.content;
  return `${formatTime(t.time)}  ${t.content}`;
}

// 自适应：高度不足时优先显示时间更近的（已按升序），完整裁剪超出部分。
// 所有行始终在文档流中渲染（保证测量稳定），超出容器的行用 visibility 隐藏，
// 不改变布局，避免测量的先有鸡先有蛋问题，也避免底部出现半截文字。
function measure() {
  const el = containerRef.value;
  if (!el) return;
  const max = el.clientHeight;
  const children = Array.from(el.querySelectorAll<HTMLElement>(".row"));
  let count = 0;
  for (const child of children) {
    // 该行底部超出容器即停止（不显示半截行）
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
  <div
    class="desktop-layer"
    :class="{ unlocked: !settings.desktopLocked }"
    :style="bgStyle"
  >
    <!-- 解锁态拖拽手柄 -->
    <div v-if="!settings.desktopLocked" class="drag-hint" data-tauri-drag-region>
      ⠿ 拖拽移动 · 边缘缩放
    </div>

    <!-- 列表：所有行渲染在流中以稳定测量，超出容器高度的行隐藏（不裁半行） -->
    <div ref="containerRef" class="rows">
      <div
        v-for="(t, i) in items"
        :key="t.id"
        class="row"
        :class="{
          hidden: i >= visibleCount,
          expired: settings.expiredHighlight && isExpired(t),
        }"
        :style="textStyle"
      >
        {{ lineText(t) }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.desktop-layer {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  padding: var(--sp-4);
  border-radius: 10px;
  border: 1px solid transparent;
  transition: border-color var(--dur-switch) var(--ease);
}
.desktop-layer.unlocked {
  border-color: rgba(79, 140, 255, 0.6);
}
.drag-hint {
  position: absolute;
  top: 6px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 11px;
  color: rgba(255, 255, 255, 0.7);
  background: rgba(79, 140, 255, 0.5);
  padding: 2px 10px;
  border-radius: 999px;
  cursor: move;
  user-select: none;
}
.rows {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 6px;
  overflow: hidden;
}
.row {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.4;
  font-weight: 500;
}
.row.hidden {
  /* 超出容器高度的行：保留在文档流以稳定测量，仅隐藏可见性 */
  visibility: hidden;
}
.row.expired {
  color: #ff8585 !important;
}
</style>
