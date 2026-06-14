// 全局响应式状态仓库（共享给主界面与桌面层）

import { reactive, ref } from "vue";
import type { Settings, Todo, DesktopBounds } from "./types";
import { DEFAULT_SETTINGS, MAX_CONTENT_LENGTH } from "./types";
import { api } from "./api";
import { newId, sortByTime } from "./utils";

export const todos = ref<Todo[]>([]);
export const settings = reactive<Settings>({ ...DEFAULT_SETTINGS });
export const desktopBounds = ref<DesktopBounds | null>(null);
export const loaded = ref(false);

/** 应用主题到 <html data-theme> */
export function applyTheme(): void {
  let mode = settings.theme;
  if (mode === "system") {
    mode = window.matchMedia("(prefers-color-scheme: light)").matches
      ? "light"
      : "dark";
  }
  document.documentElement.setAttribute("data-theme", mode);
}

export async function loadAll(): Promise<void> {
  const state = await api.loadState();
  todos.value = sortByTime(state.todos);
  Object.assign(settings, DEFAULT_SETTINGS, state.settings);
  desktopBounds.value = state.desktopBounds;
  loaded.value = true;
  applyTheme();
}

async function persistTodos(): Promise<void> {
  await api.saveTodos(todos.value);
  await api.notifyStateChanged();
}

export async function persistSettings(): Promise<void> {
  await api.saveSettings({ ...settings });
  await api.notifyStateChanged();
}

// ---------- 事项操作 ----------

export interface TodoInput {
  content: string;
  time: string; // ISO
}

export function validateInput(input: TodoInput): string | null {
  const content = input.content.trim();
  if (!content) return "内容不能为空";
  if (content.length > MAX_CONTENT_LENGTH)
    return `内容不能超过 ${MAX_CONTENT_LENGTH} 字`;
  if (!input.time || isNaN(new Date(input.time).getTime()))
    return "请选择有效的时间";
  return null;
}

export async function addTodo(input: TodoInput): Promise<void> {
  const todo: Todo = {
    id: newId(),
    time: input.time,
    content: input.content.trim(),
    status: "pending",
    createdAt: new Date().toISOString(),
    completedAt: null,
  };
  todos.value = sortByTime([...todos.value, todo]);
  await persistTodos();
}

export async function updateTodo(
  id: string,
  patch: Partial<Pick<Todo, "content" | "time">>
): Promise<void> {
  todos.value = sortByTime(
    todos.value.map((t) =>
      t.id === id
        ? {
            ...t,
            ...(patch.content !== undefined
              ? { content: patch.content.trim() }
              : {}),
            ...(patch.time !== undefined ? { time: patch.time } : {}),
          }
        : t
    )
  );
  await persistTodos();
}

export async function toggleDone(id: string): Promise<void> {
  todos.value = todos.value.map((t) => {
    if (t.id !== id) return t;
    const done = t.status === "pending";
    return {
      ...t,
      status: done ? "done" : "pending",
      completedAt: done ? new Date().toISOString() : null,
    };
  });
  await persistTodos();
}

export async function removeTodo(id: string): Promise<void> {
  todos.value = todos.value.filter((t) => t.id !== id);
  await persistTodos();
}

/** 桌面层使用：未完成事项按时间升序 */
export function pendingTodos(): Todo[] {
  return sortByTime(todos.value.filter((t) => t.status === "pending"));
}
