// 共享数据类型定义

export type TodoStatus = "pending" | "done";

export interface Todo {
  id: string;
  /** 事项时间，ISO 字符串 */
  time: string;
  /** 事项内容 */
  content: string;
  status: TodoStatus;
  /** 创建时间 ISO */
  createdAt: string;
  /** 完成时间 ISO，未完成为 null */
  completedAt: string | null;
}

export type ThemeMode = "system" | "dark" | "light";
export type DisplayFormat = "content" | "time-content";
/** 桌面层文字颜色：直接存十六进制色值 */
export type TextColorPreset = string;

/** 桌面层文字颜色预设色板 */
export const TEXT_COLOR_PRESETS: { name: string; value: string }[] = [
  { name: "纯白", value: "#ffffff" },
  { name: "雾灰", value: "#d6dae2" },
  { name: "墨黑", value: "#16161a" },
  { name: "晴空蓝", value: "#7db5ff" },
  { name: "薄荷绿", value: "#6fe0a8" },
  { name: "暖阳橙", value: "#ffb066" },
  { name: "樱粉", value: "#ff9ec4" },
  { name: "丁香紫", value: "#b69bff" },
];

export interface Settings {
  /** 桌面悬浮层显示总开关 */
  desktopEnabled: boolean;
  /** 桌面层锁定（鼠标穿透）状态 */
  desktopLocked: boolean;
  /** 背景透明度 0-100，0 全透明，100 实色 */
  backgroundOpacity: number;
  /** 开机启动 */
  autostart: boolean;
  /** 启动后最小化到托盘 */
  startMinimized: boolean;
  /** 桌面层字体大小 px */
  fontSize: number;
  /** 桌面层文字颜色预设 */
  textColor: TextColorPreset;
  /** 桌面层文字阴影/描边 */
  textShadow: boolean;
  /** 桌面层显示格式 */
  displayFormat: DisplayFormat;
  /** 过期高亮 */
  expiredHighlight: boolean;
  /** 主题 */
  theme: ThemeMode;
}

export interface DesktopBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface AppState {
  todos: Todo[];
  settings: Settings;
  desktopBounds: DesktopBounds | null;
}

export const DEFAULT_SETTINGS: Settings = {
  desktopEnabled: false,
  desktopLocked: true,
  backgroundOpacity: 0,
  autostart: false,
  startMinimized: false,
  fontSize: 16,
  textColor: "#ffffff",
  textShadow: true,
  displayFormat: "time-content",
  expiredHighlight: true,
  theme: "system",
};

export const MAX_CONTENT_LENGTH = 500;
