<script setup lang="ts">
import { nextTick, ref } from "vue";
import type { Todo } from "../types";
import { MAX_CONTENT_LENGTH } from "../types";

const props = defineProps<{
  todo: Todo;
  expired: boolean;
  timeText: string;
  toLocal: string;
}>();

const emit = defineEmits<{
  toggle: [];
  remove: [];
  edit: [content: string, timeLocal: string];
}>();

const editing = ref(false);
const confirming = ref(false);
const editContent = ref("");
const editTime = ref("");
const contentInput = ref<HTMLInputElement | null>(null);

async function startEdit() {
  if (props.todo.status === "done") return;
  editContent.value = props.todo.content;
  editTime.value = props.toLocal;
  editing.value = true;
  await nextTick();
  contentInput.value?.focus();
}

function commitEdit() {
  const c = editContent.value.trim();
  if (c) emit("edit", c, editTime.value);
  editing.value = false;
}
</script>

<template>
  <!-- 单一稳定根节点：编辑态/常规态只切换内部内容，根 DOM 不被替换，
       避免在 TransitionGroup 中触发离场动画导致取消时整条瞬时消失（#1） -->
  <div
    class="item"
    :class="{
      done: todo.status === 'done',
      expired: expired && !editing,
      'edit-mode': editing,
    }"
    @dblclick="!editing && startEdit()"
  >
    <!-- 编辑态：纵向布局，按钮单独成行 -->
    <template v-if="editing">
      <input
        ref="contentInput"
        v-model="editContent"
        class="edit-input"
        :maxlength="MAX_CONTENT_LENGTH"
        placeholder="事项内容"
        @keydown.enter="commitEdit"
        @keydown.esc="editing = false"
      />
      <div class="edit-row">
        <input v-model="editTime" class="edit-input time" type="datetime-local" />
        <div class="edit-actions">
          <button class="btn-sm ghost" @click="editing = false">取消</button>
          <button class="btn-sm primary" @click="commitEdit">保存</button>
        </div>
      </div>
    </template>

    <!-- 常规态 -->
    <template v-else>
      <!-- 复选框 -->
      <button
        class="check"
        :class="{ checked: todo.status === 'done' }"
        :title="todo.status === 'done' ? '恢复为未完成' : '标记完成'"
        @click="emit('toggle')"
      >
        <svg v-if="todo.status === 'done'" width="12" height="12" viewBox="0 0 12 12">
          <path d="M2.5 6.2 L5 8.5 L9.5 3.5" stroke="#fff" stroke-width="1.6" fill="none" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>

      <!-- 内容 -->
      <div class="body">
        <div class="text">{{ todo.content }}</div>
        <div class="meta">
          <svg width="12" height="12" viewBox="0 0 12 12" class="clock">
            <circle cx="6" cy="6" r="4.5" stroke="currentColor" stroke-width="1" fill="none" />
            <path d="M6 3.5 V6 L7.8 7.2" stroke="currentColor" stroke-width="1" fill="none" stroke-linecap="round" />
          </svg>
          <span>{{ timeText }}</span>
          <span v-if="expired" class="badge">已过期</span>
        </div>
      </div>

      <!-- 操作区 -->
      <div class="ops" v-if="!confirming">
        <button v-if="todo.status === 'pending'" class="iconbtn" title="编辑" @click="startEdit">
          <svg width="14" height="14" viewBox="0 0 14 14">
            <path d="M9.5 2.5 L11.5 4.5 L5 11 L2.5 11.5 L3 9 Z" stroke="currentColor" stroke-width="1.1" fill="none" stroke-linejoin="round" />
          </svg>
        </button>
        <button class="iconbtn danger" title="删除" @click="confirming = true">
          <svg width="14" height="14" viewBox="0 0 14 14">
            <path d="M3 4 H11 M5.5 4 V2.5 H8.5 V4 M4 4 L4.5 11.5 H9.5 L10 4" stroke="currentColor" stroke-width="1.1" fill="none" stroke-linejoin="round" />
          </svg>
        </button>
      </div>

      <!-- 删除二次确认 -->
      <div class="ops confirm" v-else>
        <span class="confirm-text">永久删除？</span>
        <button class="iconbtn danger solid" @click="emit('remove')">删除</button>
        <button class="iconbtn" @click="confirming = false">取消</button>
      </div>
    </template>
  </div>
</template>

<style scoped>
.item {
  position: relative;
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-card);
  padding: var(--sp-3);
  overflow: hidden;
  transition: background var(--dur-hover) var(--ease),
    border-color var(--dur-hover) var(--ease),
    transform var(--dur-hover) var(--ease);
}
/* 左侧强调竖条，悬停时显现，增强层次 */
.item::before {
  content: "";
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--accent-grad);
  opacity: 0;
  transition: opacity var(--dur-hover) var(--ease);
}
.item:hover {
  background: var(--bg-card-hover);
  border-color: var(--border-strong);
  transform: translateX(2px);
}
.item:hover::before {
  opacity: 0.9;
}
.item:hover .ops {
  opacity: 1;
}
.item.expired {
  border-color: rgba(255, 107, 107, 0.3);
}
.item.expired::before {
  background: var(--danger);
  opacity: 0.9;
}
.item.done::before {
  background: var(--success);
}
.check {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
  border-radius: 6px;
  border: 1.5px solid var(--text-disabled);
  background: none;
  cursor: pointer;
  display: grid;
  place-items: center;
  transition: all var(--dur-hover) var(--ease);
}
.check:hover {
  border-color: var(--accent);
}
.check.checked {
  background: var(--success);
  border-color: var(--success);
}
.body {
  flex: 1;
  min-width: 0;
}
.text {
  font-size: 14px;
  color: var(--text-primary);
  word-break: break-word;
}
.meta {
  display: flex;
  align-items: center;
  gap: 5px;
  margin-top: 3px;
  font-size: 12px;
  color: var(--text-secondary);
}
.done .text {
  color: var(--text-disabled);
  text-decoration: line-through;
}
.expired .meta {
  color: var(--danger);
}
.badge {
  background: rgba(255, 107, 107, 0.15);
  color: var(--danger);
  border-radius: var(--radius-tag);
  padding: 1px 6px;
  font-size: 11px;
}
/* 编辑态：整块纵向布局 */
.item.edit-mode {
  flex-direction: column;
  align-items: stretch;
  gap: var(--sp-2);
  background: var(--bg-card-hover);
  border-color: var(--accent);
}
.edit-row {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}
.edit-input {
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid var(--accent);
  border-radius: var(--radius-btn);
  color: var(--text-primary);
  padding: 6px var(--sp-2);
  font-size: 14px;
  outline: none;
  width: 100%;
}
.edit-input.time {
  flex: 1;
  min-width: 0;
  width: auto;
}
.edit-actions {
  display: flex;
  gap: var(--sp-2);
  flex-shrink: 0;
}
.btn-sm {
  height: 32px;
  padding: 0 var(--sp-3);
  border-radius: var(--radius-btn);
  font-size: 13px;
  cursor: pointer;
  border: 1px solid transparent;
  transition: all var(--dur-hover) var(--ease);
  white-space: nowrap;
}
.btn-sm.primary {
  background: var(--accent);
  color: #fff;
}
.btn-sm.primary:hover {
  background: var(--accent-hover);
}
.btn-sm.ghost {
  background: none;
  border-color: var(--border);
  color: var(--text-secondary);
}
.btn-sm.ghost:hover {
  color: var(--text-primary);
  background: var(--bg-card);
}
.ops {
  display: flex;
  align-items: center;
  gap: var(--sp-1);
  opacity: 0;
  transition: opacity var(--dur-hover) var(--ease);
  flex-shrink: 0;
}
.ops.confirm {
  opacity: 1;
}
.confirm-text {
  font-size: 12px;
  color: var(--text-secondary);
}
.iconbtn {
  min-width: 28px;
  height: 28px;
  padding: 0 6px;
  display: grid;
  place-items: center;
  background: none;
  border: none;
  border-radius: var(--radius-btn);
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  transition: all var(--dur-hover) var(--ease);
}
.iconbtn:hover {
  background: var(--bg-card-hover);
  color: var(--text-primary);
}
.iconbtn.danger:hover {
  color: var(--danger);
}
.iconbtn.danger.solid {
  background: var(--danger);
  color: #fff;
}
.iconbtn.ok {
  color: var(--success);
}
</style>
