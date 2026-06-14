// 时间/事项相关工具

import type { Todo } from "./types";

/** 是否已过期：时间早于现在且未完成 */
export function isExpired(todo: Todo): boolean {
  return todo.status === "pending" && new Date(todo.time).getTime() < Date.now();
}

/** 按事项时间升序（最近的在前） */
export function sortByTime(todos: Todo[]): Todo[] {
  return [...todos].sort(
    (a, b) => new Date(a.time).getTime() - new Date(b.time).getTime()
  );
}

const pad = (n: number) => String(n).padStart(2, "0");

/** 友好时间显示：今天/明天/昨天 + 时分，或 月-日 + 时分 */
export function formatTime(iso: string): string {
  const d = new Date(iso);
  if (isNaN(d.getTime())) return "";
  const now = new Date();
  const sameDay = (a: Date, b: Date) =>
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate();

  const hm = `${pad(d.getHours())}:${pad(d.getMinutes())}`;
  const tomorrow = new Date(now);
  tomorrow.setDate(now.getDate() + 1);
  const yesterday = new Date(now);
  yesterday.setDate(now.getDate() - 1);

  if (sameDay(d, now)) return `今天 ${hm}`;
  if (sameDay(d, tomorrow)) return `明天 ${hm}`;
  if (sameDay(d, yesterday)) return `昨天 ${hm}`;
  if (d.getFullYear() === now.getFullYear())
    return `${d.getMonth() + 1}月${d.getDate()}日 ${hm}`;
  return `${d.getFullYear()}/${pad(d.getMonth() + 1)}/${pad(d.getDate())} ${hm}`;
}

/** 转 <input type="datetime-local"> 需要的本地格式 yyyy-MM-ddTHH:mm */
export function toLocalInput(iso: string): string {
  const d = new Date(iso);
  if (isNaN(d.getTime())) return "";
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(
    d.getHours()
  )}:${pad(d.getMinutes())}`;
}

/** 从 datetime-local 值转 ISO */
export function fromLocalInput(v: string): string {
  return new Date(v).toISOString();
}

export function newId(): string {
  return crypto.randomUUID();
}
