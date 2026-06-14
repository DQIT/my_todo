<script setup lang="ts">
import { settings, persistSettings } from "../store";
import { api } from "../api";
import type { DisplayFormat, ThemeMode } from "../types";
import { TEXT_COLOR_PRESETS, BG_COLOR_PRESETS } from "../types";
import { applyTheme } from "../store";

// 勾选标记颜色：深色背景用白勾，浅色背景用黑勾
function checkColor(hex: string): string {
  const h = hex.replace("#", "");
  const r = parseInt(h.slice(0, 2), 16) || 0;
  const g = parseInt(h.slice(2, 4), 16) || 0;
  const b = parseInt(h.slice(4, 6), 16) || 0;
  return 0.299 * r + 0.587 * g + 0.114 * b >= 140 ? "#16161a" : "#ffffff";
}

// 每个设置变更：持久化 + 触发对应副作用
async function save() {
  await persistSettings();
}

async function onDesktopEnabled() {
  await save();
  await api.applyDesktopVisibility(settings.desktopEnabled);
}

async function onLockChange() {
  await save();
  await api.applyDesktopLock(settings.desktopLocked);
}

async function onAutostart() {
  await save();
  await api.setAutostart(settings.autostart);
}

async function onThemeChange(v: ThemeMode) {
  settings.theme = v;
  applyTheme();
  await save();
}
</script>

<template>
  <div class="panel">
    <!-- 桌面显示 -->
    <section class="group">
      <h3>桌面悬浮层</h3>
      <div class="row">
        <div class="label">
          <span>桌面显示</span>
          <small>将未完成事项贴在桌面上</small>
        </div>
        <label class="switch">
          <input type="checkbox" v-model="settings.desktopEnabled" @change="onDesktopEnabled" />
          <span class="slider" />
        </label>
      </div>

      <div class="row">
        <div class="label">
          <span>锁定桌面层</span>
          <small>锁定后鼠标穿透，融入壁纸；解锁可拖拽/缩放</small>
        </div>
        <label class="switch">
          <input type="checkbox" v-model="settings.desktopLocked" @change="onLockChange" />
          <span class="slider" />
        </label>
      </div>

      <div class="row stack">
        <div class="label"><span>背景颜色</span></div>
        <div class="swatches">
          <button
            v-for="c in BG_COLOR_PRESETS"
            :key="c.value"
            class="swatch"
            :class="{ active: settings.backgroundColor.toLowerCase() === c.value }"
            :style="{ background: c.value }"
            :title="c.name"
            @click="settings.backgroundColor = c.value; save()"
          >
            <svg v-if="settings.backgroundColor.toLowerCase() === c.value" width="12" height="12" viewBox="0 0 12 12">
              <path d="M2.5 6.2 L5 8.5 L9.5 3.5" :stroke="checkColor(c.value)" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
        </div>
      </div>

      <div class="row">
        <div class="label">
          <span>背景透明度</span>
          <small>{{ settings.backgroundOpacity }}% · 0 为全透明</small>
        </div>
        <input
          type="range" min="0" max="100" class="range"
          v-model.number="settings.backgroundOpacity" @change="save"
        />
      </div>

      <div class="row">
        <div class="label"><span>字体大小</span><small>{{ settings.fontSize }}px</small></div>
        <input
          type="range" min="12" max="32" class="range"
          v-model.number="settings.fontSize" @change="save"
        />
      </div>

      <div class="row stack">
        <div class="label"><span>文字颜色</span></div>
        <div class="swatches">
          <button
            v-for="c in TEXT_COLOR_PRESETS"
            :key="c.value"
            class="swatch"
            :class="{ active: settings.textColor.toLowerCase() === c.value }"
            :style="{ background: c.value }"
            :title="c.name"
            @click="settings.textColor = c.value; save()"
          >
            <svg v-if="settings.textColor.toLowerCase() === c.value" width="12" height="12" viewBox="0 0 12 12">
              <path d="M2.5 6.2 L5 8.5 L9.5 3.5" :stroke="checkColor(c.value)" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
        </div>
      </div>

      <div class="row">
        <div class="label"><span>文字阴影</span><small>提升任意壁纸下的可读性</small></div>
        <label class="switch">
          <input type="checkbox" v-model="settings.textShadow" @change="save" />
          <span class="slider" />
        </label>
      </div>

      <div class="row">
        <div class="label"><span>显示格式</span></div>
        <div class="segmented">
          <button :class="{ active: settings.displayFormat === 'time-content' }"
            @click="settings.displayFormat = 'time-content' as DisplayFormat; save()">时间+内容</button>
          <button :class="{ active: settings.displayFormat === 'content' }"
            @click="settings.displayFormat = 'content' as DisplayFormat; save()">仅内容</button>
        </div>
      </div>

      <div class="row">
        <div class="label"><span>过期高亮</span></div>
        <label class="switch">
          <input type="checkbox" v-model="settings.expiredHighlight" @change="save" />
          <span class="slider" />
        </label>
      </div>
    </section>

    <!-- 通用 -->
    <section class="group">
      <h3>通用</h3>
      <div class="row">
        <div class="label"><span>主题</span></div>
        <div class="segmented">
          <button :class="{ active: settings.theme === 'system' }" @click="onThemeChange('system')">跟随系统</button>
          <button :class="{ active: settings.theme === 'dark' }" @click="onThemeChange('dark')">深色</button>
          <button :class="{ active: settings.theme === 'light' }" @click="onThemeChange('light')">浅色</button>
        </div>
      </div>
      <div class="row">
        <div class="label"><span>开机启动</span><small>随 Windows 启动自动运行</small></div>
        <label class="switch">
          <input type="checkbox" v-model="settings.autostart" @change="onAutostart" />
          <span class="slider" />
        </label>
      </div>
      <div class="row">
        <div class="label"><span>启动后最小化</span><small>启动时静默驻留托盘</small></div>
        <label class="switch">
          <input type="checkbox" v-model="settings.startMinimized" @change="save" />
          <span class="slider" />
        </label>
      </div>
    </section>
  </div>
</template>

<style scoped>
.panel {
  height: 100%;
  overflow-y: auto;
  padding: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-6);
}
.group h3 {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: var(--sp-3);
  letter-spacing: 0.3px;
}
.row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-4);
  padding: var(--sp-3) 0;
  border-bottom: 1px solid var(--border);
}
.row:last-child {
  border-bottom: none;
}
.label {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.label span {
  font-size: 14px;
}
.label small {
  font-size: 12px;
  color: var(--text-secondary);
}
/* 开关 */
.switch {
  position: relative;
  width: 40px;
  height: 22px;
  flex-shrink: 0;
}
.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}
.slider {
  position: absolute;
  inset: 0;
  background: var(--bg-card-hover);
  border: 1px solid var(--border);
  border-radius: 999px;
  cursor: pointer;
  transition: background var(--dur-switch) var(--ease);
}
.slider::before {
  content: "";
  position: absolute;
  width: 16px;
  height: 16px;
  left: 2px;
  top: 2px;
  border-radius: 50%;
  background: var(--text-secondary);
  transition: transform var(--dur-switch) var(--ease),
    background var(--dur-switch) var(--ease);
}
.switch input:checked + .slider {
  background: var(--accent);
  border-color: var(--accent);
}
.switch input:checked + .slider::before {
  transform: translateX(18px);
  background: #fff;
}
/* 滑块 */
.range {
  width: 180px;
  accent-color: var(--accent);
  cursor: pointer;
}
/* 分段控件 */
.segmented {
  display: flex;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-btn);
  overflow: hidden;
}
.segmented button {
  background: none;
  border: none;
  color: var(--text-secondary);
  padding: 6px var(--sp-3);
  font-size: 13px;
  cursor: pointer;
  transition: all var(--dur-hover) var(--ease);
}
.segmented button:hover {
  color: var(--text-primary);
}
.segmented button.active {
  background: var(--accent);
  color: #fff;
}
/* 颜色色板 */
.row.stack {
  flex-direction: column;
  align-items: stretch;
  gap: var(--sp-3);
}
.swatches {
  display: flex;
  gap: var(--sp-2);
  width: 100%;
}
.swatch {
  flex: 1;
  aspect-ratio: 1;
  max-width: 30px;
  border-radius: 50%;
  border: 1px solid var(--border);
  cursor: pointer;
  display: grid;
  place-items: center;
  padding: 0;
  transition: transform var(--dur-hover) var(--ease),
    box-shadow var(--dur-hover) var(--ease);
}
.swatch:hover {
  transform: scale(1.12);
}
.swatch.active {
  box-shadow: 0 0 0 2px var(--bg-window), 0 0 0 4px var(--accent);
}
</style>
