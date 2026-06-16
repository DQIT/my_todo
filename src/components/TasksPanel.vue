<script setup lang="ts">
import { computed, ref } from "vue";
import {
  todos,
  addTodo,
  toggleDone,
  removeTodo,
  updateTodo,
  validateInput,
} from "../store";
import { isExpired, formatTime, toLocalInput, fromLocalInput } from "../utils";
import { MAX_CONTENT_LENGTH } from "../types";
import TodoItem from "./TodoItem.vue";

type Filter = "all" | "pending" | "done";
const filter = ref<Filter>("all");
const keyword = ref("");

// 新建表单
const newContent = ref("");
const defaultTime = () => {
  const d = new Date();
  d.setMinutes(d.getMinutes() + 60, 0, 0);
  return toLocalInput(d.toISOString());
};
const newTime = ref(defaultTime());
const formError = ref<string | null>(null);

const filtered = computed(() => {
  const kw = keyword.value.trim().toLowerCase();
  return todos.value.filter((t) => {
    if (filter.value === "pending" && t.status !== "pending") return false;
    if (filter.value === "done" && t.status !== "done") return false;
    if (kw && !t.content.toLowerCase().includes(kw)) return false;
    return true;
  });
});

const counts = computed(() => ({
  all: todos.value.length,
  pending: todos.value.filter((t) => t.status === "pending").length,
  done: todos.value.filter((t) => t.status === "done").length,
}));

async function submit() {
  const input = { content: newContent.value, time: fromLocalInput(newTime.value) };
  const err = validateInput(input);
  if (err) {
    formError.value = err;
    return;
  }
  formError.value = null;
  await addTodo(input);
  newContent.value = "";
  newTime.value = defaultTime();
}

async function onEdit(id: string, content: string, timeLocal: string) {
  await updateTodo(id, { content, time: fromLocalInput(timeLocal) });
}
</script>

<template>
  <div class="panel">
    <!-- 新建区 -->
    <div class="composer">
      <input
        v-model="newContent"
        class="input content"
        type="text"
        :maxlength="MAX_CONTENT_LENGTH"
        placeholder="添加一个待办事项…"
        @keydown.enter="submit"
      />
      <div class="composer-row">
        <input v-model="newTime" class="input time" type="datetime-local" />
        <button class="btn-primary" @click="submit">添加</button>
      </div>
    </div>
    <div v-if="formError" class="form-error">{{ formError }}</div>

    <!-- 过滤与搜索 -->
    <div class="toolbar">
      <div class="filters">
        <button
          v-for="f in (['all', 'pending', 'done'] as Filter[])"
          :key="f"
          class="chip"
          :class="{ active: filter === f }"
          @click="filter = f"
        >
          {{ f === "all" ? "全部" : f === "pending" ? "未完成" : "已完成" }}
          <span class="count">{{ counts[f] }}</span>
        </button>
      </div>
      <div class="search">
        <svg width="14" height="14" viewBox="0 0 14 14" class="sicon">
          <circle cx="6" cy="6" r="4.2" stroke="currentColor" stroke-width="1.2" fill="none" />
          <path d="M9.2 9.2 L12 12" stroke="currentColor" stroke-width="1.2" />
        </svg>
        <input v-model="keyword" type="text" placeholder="搜索内容" />
      </div>
    </div>

    <!-- 列表 -->
    <div class="list" v-if="filtered.length">
      <TransitionGroup name="list">
        <TodoItem
          v-for="t in filtered"
          :key="t.id"
          :todo="t"
          :expired="isExpired(t)"
          :time-text="formatTime(t.time)"
          :to-local="toLocalInput(t.time)"
          @toggle="toggleDone(t.id)"
          @remove="removeTodo(t.id)"
          @edit="(c, tm) => onEdit(t.id, c, tm)"
        />
      </TransitionGroup>
    </div>
    <div class="empty" v-else>
      <div class="empty-icon">✓</div>
      <p>{{ keyword || filter !== "all" ? "没有符合条件的事项" : "还没有待办，添加一个开始吧" }}</p>
    </div>
  </div>
</template>

<style scoped>
.panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: var(--sp-4);
  gap: var(--sp-3);
  min-height: 0;
}
.composer {
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  padding: var(--sp-3);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}
.composer-row {
  display: flex;
  gap: var(--sp-2);
}
.input {
  background: var(--bg-elevated);
  border: 1px solid var(--border);
  border-radius: var(--radius-btn);
  color: var(--text-primary);
  padding: var(--sp-2) var(--sp-3);
  font-size: 14px;
  outline: none;
  transition: border-color var(--dur-hover) var(--ease),
    box-shadow var(--dur-hover) var(--ease);
}
.input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px rgba(79, 140, 255, 0.18);
}
.input.content {
  flex: 1;
}
.input.time {
  flex: 1;
  min-width: 0;
}
.btn-primary {
  background: var(--accent-grad);
  color: #fff;
  border: none;
  border-radius: var(--radius-btn);
  padding: 0 var(--sp-4);
  height: 36px;
  flex-shrink: 0;
  white-space: nowrap;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  box-shadow: 0 6px 18px rgba(79, 140, 255, 0.3);
  transition: transform var(--dur-hover) var(--ease),
    box-shadow var(--dur-hover) var(--ease);
}
.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 10px 24px rgba(79, 140, 255, 0.42);
}
.btn-primary:active {
  transform: translateY(0);
}
.form-error {
  color: var(--danger);
  font-size: 12px;
  margin-top: -6px;
}
.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--sp-3);
}
.filters {
  display: flex;
  gap: var(--sp-1);
}
.chip {
  background: var(--bg-card);
  border: 1px solid transparent;
  color: var(--text-secondary);
  border-radius: 999px;
  padding: 5px var(--sp-3);
  font-size: 13px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all var(--dur-hover) var(--ease);
}
.chip:hover {
  color: var(--text-primary);
}
.chip.active {
  color: var(--accent);
  border-color: var(--accent);
  background: rgba(79, 140, 255, 0.1);
}
.chip .count {
  font-size: 11px;
  opacity: 0.7;
}
.search {
  display: flex;
  align-items: center;
  gap: 6px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-card);
  padding: 5px var(--sp-3);
  color: var(--text-secondary);
}
.search input {
  background: none;
  border: none;
  outline: none;
  color: var(--text-primary);
  font-size: 13px;
  width: 140px;
}
.sicon {
  flex-shrink: 0;
}
.list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: var(--sp-2);
  padding-right: 4px;
  min-height: 0;
}
.empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--sp-3);
  color: var(--text-disabled);
}
.empty-icon {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  display: grid;
  place-items: center;
  background:
    radial-gradient(circle at 50% 35%, rgba(79, 140, 255, 0.22), transparent 70%),
    var(--bg-card);
  border: 1px solid var(--border);
  font-size: 26px;
  color: var(--accent);
}
.list-enter-active,
.list-leave-active {
  transition: all var(--dur-switch) var(--ease);
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-8px);
}
.list-leave-active {
  position: absolute;
}
</style>
