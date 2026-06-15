// API 桥接层
// 在 Tauri 环境下调用 Rust 后端命令；在普通浏览器（mac 预览）下用 localStorage mock。
// 这样 UI 与业务逻辑可在 mac 上 `npm run dev` 验证，Windows 专有能力在真机调试。

import type { AppState, Settings, Todo, DesktopBounds } from "./types";
import { DEFAULT_SETTINGS } from "./types";

// ---------- 项目信息（关于页 / 检查更新共用）----------
export const REPO_OWNER = "DQIT";
export const REPO_NAME = "my_todo";
export const REPO_URL = `https://github.com/${REPO_OWNER}/${REPO_NAME}`;
export const DEVELOPER_NAME = "DQIT";
export const DEVELOPER_URL = `https://github.com/${REPO_OWNER}`;
const RELEASES_URL = `${REPO_URL}/releases`;
const LATEST_RELEASE_API = `https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest`;

export interface UpdateInfo {
  /** 是否有更高版本 */
  hasUpdate: boolean;
  /** 最新版本号（去掉前缀 v） */
  latest: string;
  /** 当前版本号 */
  current: string;
  /** 发布页地址 */
  url: string;
}

const isTauri = (): boolean =>
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

// ---------- Tauri 实现 ----------
async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(cmd, args);
}

// ---------- 浏览器 mock 实现 ----------
const MOCK_KEY = "mytodo:state";

function mockLoad(): AppState {
  try {
    const raw = localStorage.getItem(MOCK_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as Partial<AppState>;
      return {
        todos: parsed.todos ?? [],
        settings: { ...DEFAULT_SETTINGS, ...(parsed.settings ?? {}) },
        desktopBounds: parsed.desktopBounds ?? null,
      };
    }
  } catch {
    // 损坏则回退默认
  }
  // 首次：塞几条示例，便于预览
  const now = Date.now();
  const sample: AppState = {
    todos: [
      mkTodo("完成季度项目结案报告", new Date(now + 2 * 864e5).toISOString()),
      mkTodo("预约牙医复诊", new Date(now + 5 * 36e5).toISOString()),
      mkTodo("回复客户邮件并确认下周会议时间", new Date(now - 36e5).toISOString()),
    ],
    settings: { ...DEFAULT_SETTINGS, desktopEnabled: true, backgroundOpacity: 20 },
    desktopBounds: null,
  };
  mockSave(sample);
  return sample;
}

function mkTodo(content: string, time: string): Todo {
  return {
    id: crypto.randomUUID(),
    time,
    content,
    status: "pending",
    createdAt: new Date().toISOString(),
    completedAt: null,
  };
}

function mockSave(state: AppState): void {
  localStorage.setItem(MOCK_KEY, JSON.stringify(state));
  // 通知其它标签页（模拟桌面层实时同步）
  window.dispatchEvent(new CustomEvent("mock-state-changed"));
}

// ---------- 统一 API ----------

export const api = {
  isTauri,

  async loadState(): Promise<AppState> {
    if (isTauri()) return invoke<AppState>("load_state");
    return mockLoad();
  },

  async saveTodos(todos: Todo[]): Promise<void> {
    if (isTauri()) {
      await invoke("save_todos", { todos });
      return;
    }
    const s = mockLoad();
    s.todos = todos;
    mockSave(s);
  },

  async saveSettings(settings: Settings): Promise<void> {
    if (isTauri()) {
      await invoke("save_settings", { settings });
      return;
    }
    const s = mockLoad();
    s.settings = settings;
    mockSave(s);
  },

  async saveDesktopBounds(bounds: DesktopBounds): Promise<void> {
    if (isTauri()) {
      await invoke("save_desktop_bounds", { bounds });
      return;
    }
    const s = mockLoad();
    s.desktopBounds = bounds;
    mockSave(s);
  },

  // ---- 桌面层控制（仅 Tauri 生效，浏览器为 no-op）----
  async applyDesktopVisibility(enabled: boolean): Promise<void> {
    if (isTauri()) await invoke("set_desktop_visible", { enabled });
  },

  async applyDesktopLock(locked: boolean): Promise<void> {
    if (isTauri()) await invoke("set_desktop_locked", { locked });
  },

  async setAutostart(enabled: boolean): Promise<void> {
    if (isTauri()) await invoke("set_autostart", { enabled });
  },

  async showMainWindow(): Promise<void> {
    if (isTauri()) await invoke("show_main_window");
  },

  async hideMainWindow(): Promise<void> {
    if (isTauri()) await invoke("hide_main_window");
  },

  async quitApp(): Promise<void> {
    if (isTauri()) await invoke("quit_app");
  },

  /** 用系统浏览器打开外部链接 */
  async openUrl(url: string): Promise<void> {
    if (isTauri()) {
      await invoke("open_url", { url });
    } else {
      window.open(url, "_blank", "noopener,noreferrer");
    }
  },

  /** 查询 GitHub 最新 Release，与当前版本比较 */
  async checkUpdate(current: string): Promise<UpdateInfo> {
    const res = await fetch(LATEST_RELEASE_API, {
      headers: { Accept: "application/vnd.github+json" },
    });
    if (!res.ok) {
      // 还没有任何 Release（404）也视为"已是最新"，但抛出可识别错误供 UI 区分
      throw new Error(`GitHub API ${res.status}`);
    }
    const data = (await res.json()) as { tag_name?: string; html_url?: string };
    const latest = (data.tag_name ?? "").replace(/^v/i, "").trim();
    return {
      latest: latest || current,
      current,
      hasUpdate: latest ? compareVersion(latest, current) > 0 : false,
      url: data.html_url || RELEASES_URL,
    };
  },

  // ---- 事件订阅：后端状态变更 → 前端刷新 ----
  async onStateChanged(handler: () => void): Promise<() => void> {
    if (isTauri()) {
      const { listen } = await import("@tauri-apps/api/event");
      const un = await listen("state-changed", () => handler());
      return un;
    }
    // 浏览器：监听自定义事件 + storage（跨标签页）
    const fn = () => handler();
    window.addEventListener("mock-state-changed", fn);
    window.addEventListener("storage", fn);
    return () => {
      window.removeEventListener("mock-state-changed", fn);
      window.removeEventListener("storage", fn);
    };
  },

  /** 通知后端广播状态变更（让另一窗口刷新） */
  async notifyStateChanged(): Promise<void> {
    if (isTauri()) await invoke("broadcast_state_changed");
    else window.dispatchEvent(new CustomEvent("mock-state-changed"));
  },
};

/** 语义化版本比较：a>b 返回正数，a<b 返回负数，相等返回 0 */
function compareVersion(a: string, b: string): number {
  const pa = a.split(".").map((n) => parseInt(n, 10) || 0);
  const pb = b.split(".").map((n) => parseInt(n, 10) || 0);
  const len = Math.max(pa.length, pb.length);
  for (let i = 0; i < len; i++) {
    const diff = (pa[i] ?? 0) - (pb[i] ?? 0);
    if (diff !== 0) return diff;
  }
  return 0;
}
