<script setup lang="ts">
const version = "1.0.0";
</script>

<template>
  <div class="panel">
    <div class="logo">
      <span class="dot" />
    </div>
    <h1>MyToDo</h1>
    <p class="ver">版本 v{{ version }}</p>
    <p class="desc">一款将待办清单像壁纸一样贴在桌面上的轻量工具，<br />让你在不打断工作流的前提下持续掌握任务。</p>
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
.meta {
  display: flex;
  gap: var(--sp-2);
  color: var(--text-disabled);
  font-size: 12px;
  margin-top: var(--sp-4);
}
</style>
